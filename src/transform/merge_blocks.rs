use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeBlocks {
    pub from: RegexSet,
    pub to: String,
    pub main: Option<RegexSet>,
    #[serde(default)]
    pub check: CheckLevel,
}

impl MergeBlocks {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let groups = match_groups(ir.blocks.keys().cloned(), &self.from, &self.to);

        for (to, group) in groups {
            info!("Merging blocks, dest: {}", to);
            for id in &group {
                info!("   {}", id);
            }
            self.merge_blocks(ir, group, to, self.main.as_ref())?;
        }

        Ok(())
    }

    fn merge_blocks(
        &self,
        ir: &mut IR,
        ids: BTreeSet<String>,
        to: String,
        main: Option<&RegexSet>,
    ) -> anyhow::Result<()> {
        let mut main_id = ids.iter().next().unwrap().clone();
        if let Some(main) = main {
            for id in ids.iter() {
                if main.is_match(id) {
                    main_id = id.clone();
                    break;
                }
            }
        }
        let b = ir.blocks.get(&main_id).unwrap().clone();

        // todo
        //for id in &ids {
        //    let b2 = ir.blocks.get(id).unwrap();
        //    check_mergeable_blocks(&b, b2, self.check)?;
        //}

        replace_block_ids(ir, &ids, to.clone());
        for id in &ids {
            ir.blocks.remove(id);
        }
        ir.blocks.insert(to, b);

        Ok(())
    }
}
