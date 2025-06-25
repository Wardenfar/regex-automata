use std::collections::HashSet;

use itertools::Itertools;
use regex_syntax::hir::{Class, ClassUnicode, ClassUnicodeRange, Hir, Repetition};

use crate::*;

/// Convert DFA back to REGEX Syntax
pub fn dfa_to_hir(dfa_origin: &Dfa<char>) -> Hir {
    let mut dfa = Dfa {
        accept_states: HashSet::default(),
        initial_states: HashSet::default(),
        links: dfa_origin
            .links
            .iter()
            .map(|link| Link {
                from: link.from,
                to: link.to,
                symbol: Hir::class(Class::Unicode(ClassUnicode::new([ClassUnicodeRange::new(
                    link.symbol,
                    link.symbol,
                )]))),
            })
            .collect(),
    };

    let next_counter_state = dfa_origin.max_state().map(|x| x + 1).unwrap_or(0);
    let counter = Counter::new(next_counter_state);

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
        debug_assert!(dfa.links_from_to(rip, rip).count() <= 1);

        let self_loop = dfa.links_from_to(rip, rip).exactly_one().ok();
        let self_loop = self_loop.map(|self_loop| {
            Hir::repetition(Repetition {
                greedy: false,
                min: 0,
                max: None,
                sub: Box::new(self_loop.symbol.clone()),
            })
        });

        let incomings_groups = dfa.links_to(rip).cloned().into_group_map_by(|x| x.from);
        let outgoings_groups = dfa.links_from(rip).cloned().into_group_map_by(|x| x.to);

        dfa.remove_links_any(rip);

        debug_assert_eq!(dfa.links_from_to(rip, rip).count(), 0);

        for (from, incomings) in &incomings_groups {
            for (to, outgoings) in &outgoings_groups {
                if *from == rip || *to == rip {
                    continue;
                }
                debug_assert_ne!(from, to);

                let incomings = incomings.iter().cloned().map(|l| l.symbol).collect();
                let outgoings = outgoings.iter().cloned().map(|l| l.symbol).collect();

                let in_sym = Hir::alternation(incomings);
                let out_sym = Hir::alternation(outgoings);

                let items = match self_loop.as_ref().cloned() {
                    Some(self_loop) => vec![in_sym, self_loop, out_sym],
                    None => vec![in_sym, out_sym],
                };
                let hir = Hir::concat(items);

                dfa.link(*from, *to, hir);
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
