//! The implementation of a high-level intermediate representation (HIR) combiner.

use std::collections::HashSet;

use crate::{syntax::ast::Ast, constants::CONTRACT_PART_SEPARATOR};

use super::{Hir, Root};

/// A high-level intermediate representation (HIR) combiner.
///
/// It takes a vector of HIRs and combines them into a single HIR
/// by appending the function nodes to the root contract node.
pub struct Combiner {}

impl Combiner {
    /// Creates a new combiner.
    #[must_use]
    pub fn new() -> Self {
        Combiner {}
    }

    /// Verifies that the ASTs are valid.
    ///
    /// This function is called before the translation to HIR.
    pub fn verify(&self, asts: &Vec<Ast>) -> anyhow::Result<(), String> {
        if let Some(Ast::Root(first_root)) = asts.first() {
            let first_contract_name = &first_root.contract_name;
            let first_parts: Vec<&str> = first_contract_name
                .split(CONTRACT_PART_SEPARATOR)
                .collect();

            for ast in asts {
                if let Ast::Root(root) = ast {
                    let parts: Vec<&str> =
                        root.contract_name.split(CONTRACT_PART_SEPARATOR).collect();
                    if parts[0] != first_parts[0] {
                        return Err(format!(
                            "Contract name mismatch: expected '{}', found '{}'",
                            first_contract_name, root.contract_name
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Combines the translated HIRs into a single HIR. HIRs are merged by
    /// iterating over each HIR and merge their children into the root,
    /// while filtering out duplicate modifiers.
    ///
    /// This function is called after the ASTs are translated to HIR.
    pub fn combine(&self, hirs: &[Hir]) -> Hir {
        let mut root: Root = Root::default();
        let mut added_modifiers = HashSet::new();

        for hir in hirs {
            match hir {
                Hir::Root(r) => {
                    // If child is of type FunctionDefinition with the same identifier as a child of another root of ty
                    // Modifier, then they are duplicates. Traverse all children of the root and remove the duplicates.
                    for child in &r.children {
                        match child {
                            Hir::FunctionDefinition(func_def)
                                if func_def.ty == super::FunctionTy::Modifier =>
                            {
                                if added_modifiers.insert(func_def.identifier.clone()) {
                                    root.children.push(child.clone());
                                }
                            }
                            _ => root.children.push(child.clone()),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Hir::Root(root)
    }
}
