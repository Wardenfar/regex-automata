use itertools::Itertools;
use regex_syntax::hir::{
    Class, ClassBytes, ClassBytesRange, ClassUnicode, ClassUnicodeRange, Hir, Repetition,
};
use rustc_hash::FxHashMap;

use crate::*;

pub trait IntoHir {
    fn into_hir(self) -> Hir;
}

impl IntoHir for char {
    fn into_hir(self) -> Hir {
        Hir::class(Class::Unicode(ClassUnicode::new([ClassUnicodeRange::new(
            self, self,
        )])))
    }
}

impl IntoHir for u8 {
    fn into_hir(self) -> Hir {
        Hir::class(Class::Bytes(ClassBytes::new([ClassBytesRange::new(
            self, self,
        )])))
    }
}

/// Convert DFA back to REGEX Syntax
pub fn dfa_to_hir<T: IntoHir>(dfa_origin: Dfa<T>) -> Hir {
    let counter = dfa_origin.next_counter();

    let mut dfa = Dfa {
        accept_states: Default::default(),
        initial_states: Default::default(),
        links: dfa_origin
            .links
            .into_iter()
            .map(|link| Link {
                from: link.from,
                to: link.to,
                symbol: link.symbol.into_hir(),
            })
            .collect(),
    };

    let start = counter.next();
    let end = counter.next();

    for init_state in &dfa_origin.initial_states {
        dfa.link(start, *init_state, Hir::empty());
    }

    for accept_state in &dfa_origin.accept_states {
        dfa.link(*accept_state, end, Hir::empty());
    }

    merge_sibling_edges(&mut dfa);

    let mut all_states = dfa.states_set();
    all_states.remove(&start);
    all_states.remove(&end);

    for rip in all_states {
        let mut self_loops = dfa
            .links_from_to(rip, rip)
            .map(|self_loop| {
                Hir::repetition(Repetition {
                    greedy: false,
                    min: 0,
                    max: None,
                    sub: Box::new(self_loop.symbol.clone()),
                })
            })
            .collect_vec();

        let self_loop = match self_loops.len() {
            0 => None,
            1 => Some(self_loops.remove(0)),
            _ => Some(Hir::alternation(self_loops)),
        };

        let mut incomings_groups = FxHashMap::<_, Vec<_>>::default();
        for link in dfa.links_to(rip).cloned() {
            incomings_groups.entry(link.from).or_default().push(link);
        }

        let mut outgoings_groups = FxHashMap::<_, Vec<_>>::default();
        for link in dfa.links_from(rip).cloned() {
            outgoings_groups.entry(link.to).or_default().push(link);
        }

        dfa.remove_links_any(rip);

        debug_assert_eq!(dfa.links_from(rip).count(), 0);
        debug_assert_eq!(dfa.links_to(rip).count(), 0);

        for (from, incomings) in &incomings_groups {
            for (to, outgoings) in &outgoings_groups {
                if *from == rip || *to == rip {
                    continue;
                }

                let incomings = incomings.iter().cloned().map(|l| l.symbol).collect();
                let outgoings = outgoings.iter().cloned().map(|l| l.symbol).collect();

                let in_sym = Hir::alternation(incomings);
                let out_sym = Hir::alternation(outgoings);

                let items = if let Some(self_loop) = self_loop.as_ref() {
                    vec![in_sym, self_loop.clone(), out_sym]
                } else {
                    vec![in_sym, out_sym]
                };
                dfa.link(*from, *to, Hir::concat(items));
            }
        }

        merge_sibling_edges(&mut dfa);

        debug_assert_eq!(dfa.links_from_to(rip, rip).count(), 0);
    }

    assert_eq!(dfa.links.len(), 1);

    dfa.links.remove(0).symbol
}

fn merge_sibling_edges(dfa: &mut Automata<Hir>) {
    for states in dfa.states_set().into_iter().permutations(2) {
        let [from, to] = states.as_slice().try_into().unwrap();

        let hirs = dfa
            .links_from_to(from, to)
            .map(|l| l.symbol.clone())
            .collect_vec();

        if hirs.len() <= 1 {
            continue;
        }

        dfa.patch_links(from, to, Hir::alternation(hirs));
    }
}
