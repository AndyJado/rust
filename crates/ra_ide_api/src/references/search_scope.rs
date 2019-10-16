//! FIXME: write short doc here

use std::collections::HashSet;

use hir::{DefWithBody, HasSource, ModuleSource};
use ra_db::{FileId, SourceDatabase, SourceDatabaseExt};
use ra_syntax::{AstNode, TextRange};

use crate::db::RootDatabase;

use super::{NameDefinition, NameKind};

impl NameDefinition {
    pub(crate) fn search_scope(&self, db: &RootDatabase) -> HashSet<(FileId, Option<TextRange>)> {
        let module_src = self.container.definition_source(db);
        let file_id = module_src.file_id.original_file(db);

        if let NameKind::Pat((def, _)) = self.kind {
            let range = match def {
                DefWithBody::Function(f) => f.source(db).ast.syntax().text_range(),
                DefWithBody::Const(c) => c.source(db).ast.syntax().text_range(),
                DefWithBody::Static(s) => s.source(db).ast.syntax().text_range(),
            };
            return [(file_id, Some(range))].iter().cloned().collect();
        }

        if let Some(ref vis) = self.visibility {
            let vis = vis.syntax().to_string();

            // FIXME: add "pub(in path)"

            if vis.as_str() == "pub(super)" {
                if let Some(parent_module) = self.container.parent(db) {
                    let mut files = HashSet::new();

                    let parent_src = parent_module.definition_source(db);
                    let file_id = parent_src.file_id.original_file(db);

                    match parent_src.ast {
                        ModuleSource::Module(m) => {
                            let range = Some(m.syntax().text_range());
                            files.insert((file_id, range));
                        }
                        ModuleSource::SourceFile(_) => {
                            files.insert((file_id, None));
                            files.extend(
                                parent_module
                                    .children(db)
                                    .map(|m| {
                                        let src = m.definition_source(db);
                                        (src.file_id.original_file(db), None)
                                    })
                                    .collect::<HashSet<_>>(),
                            );
                        }
                    }
                    return files;
                } else {
                    let range = match module_src.ast {
                        ModuleSource::Module(m) => Some(m.syntax().text_range()),
                        ModuleSource::SourceFile(_) => None,
                    };
                    return [(file_id, range)].iter().cloned().collect();
                }
            }

            let source_root_id = db.file_source_root(file_id);
            let source_root = db.source_root(source_root_id);
            let mut files = source_root.walk().map(|id| (id.into(), None)).collect::<HashSet<_>>();

            if vis.as_str() == "pub(crate)" {
                return files;
            }
            if vis.as_str() == "pub" {
                let krate = self.container.krate(db).unwrap();
                let crate_graph = db.crate_graph();

                for crate_id in crate_graph.iter() {
                    let mut crate_deps = crate_graph.dependencies(crate_id);

                    if crate_deps.any(|dep| dep.crate_id() == krate.crate_id()) {
                        let root_file = crate_graph.crate_root(crate_id);
                        let source_root_id = db.file_source_root(root_file);
                        let source_root = db.source_root(source_root_id);
                        files.extend(source_root.walk().map(|id| (id.into(), None)));
                    }
                }

                return files;
            }
        }

        let range = match module_src.ast {
            ModuleSource::Module(m) => Some(m.syntax().text_range()),
            ModuleSource::SourceFile(_) => None,
        };
        [(file_id, range)].iter().cloned().collect()
    }
}
