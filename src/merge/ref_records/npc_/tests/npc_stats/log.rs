use super::{assert_eq, *};

test_log!(
    npc_,
    Npc,
    "Merged NPC_ record: \"\"\n",
    "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"data.stats.speed\": 0 -> 1 [\"Plugin1.esp\"]\n\"data.stats.personality\": 0 -> 1 [\"Plugin1.esp\"]\n\"data.stats.medium_armor\": 0 -> 1 [\"Plugin1.esp\"]\n\"data.stats.speechcraft\": 3 -> 2 [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
    values_some_npc_data_skills_attributes:data:stats
);

#[test]
fn merge_auto_to_manual() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    cfg.guts.verboseness_details_merge_record_merged = 1;
    cfg.guts.verboseness_details_merge_field_changed = 2;
    cfg.guts.debug_level_merge_compare_to_the_last = 1;
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
    assert_eq!(
                log.test_file(),
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"data.stats\": \"Auto\" -> \"Manual\" [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n"
            );
    assert_eq!(log.test_text(), "");
}

#[test]
fn merge_manual_to_auto() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    cfg.guts.verboseness_details_merge_record_merged = 1;
    cfg.guts.verboseness_details_merge_field_changed = 2;
    cfg.guts.debug_level_merge_compare_to_the_last = 1;
    src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
    src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[2].data.stats = values_some_npc_data_skills_attributes[0].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
    assert_eq!(
                log.test_file(),
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"npc_flags\": NpcFlags(0x0) -> NpcFlags(AUTO_CALCULATE) [\"Plugin1.esp\"]\n\"data.stats\": \"Manual\" -> \"Auto\" [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n"
            );
    assert_eq!(log.test_text(), "");
}
