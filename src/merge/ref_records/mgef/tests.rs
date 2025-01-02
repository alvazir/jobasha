use super::{
    assert_eq_inner, merge_mgef, test_basic, test_debug, test_debug_all_equal, test_debug_compare_to_the_last,
    test_debug_equal_to_the_last, test_debug_list_all_plugins, test_debug_single, test_flags, test_init, test_log, test_log_flags,
    test_merge, MergeLog, RawPlugin,
};
use crate::{Cfg, IntermediateRecords, PluginInfo};
use paste::paste;
use pretty_assertions::assert_eq;
use std::iter::repeat;
use tes3::esp::{EffectId, EffectSchool, MagicEffect, MagicEffectFlags, ObjectFlags, TES3Object};

mod basic {
    use super::{assert_eq, *};

    mod object_flags {
        use super::{assert_eq, *};
        test_basic!(mgef, MagicEffect, values_object_flags:flags; id=effect_id=>EffectId::Levitate=>EffectId::RemoveCurse);
        test_flags!(mgef, MagicEffect, values_object_flags:flags; id=effect_id=>EffectId::ResistShock=>EffectId::SummonSkeleton);
        test_log_flags!(
            mgef,
            MagicEffect,
            false,
            "Merged MGEF record: Recall\n",
            "Merging MGEF record: Recall [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"flags\": - DELETED [\"Plugin1.esp\"]\n\"flags\": - PERSISTENT [\"Plugin1.esp\"]\n\"flags\": + IGNORED [\"Plugin1.esp\"]\nMerged MGEF record: Recall\n",
            values_object_flags:flags,
            (effect_id = EffectId::Recall)
        );
    }

    mod cast_visual {
        use super::{assert_eq, *};
        test_basic!(mgef, MagicEffect, values_string:cast_visual; id=effect_id=>EffectId::Shield=>EffectId::DrainMagicka);
    }

    mod data_school {
        use super::{assert_eq, *};
        test_basic!(mgef, MagicEffect, values_effect_school:data:school; id=effect_id=>EffectId::Poison=>EffectId::WeaknessToCorprus);
    }

    mod data_speed {
        use super::{assert_eq, *};
        test_basic!(mgef, MagicEffect, values_f32:data:speed; id=effect_id=>EffectId::Charm=>EffectId::Sound);
    }

    mod magic_effect_flags {
        use super::{assert_eq, *};
        test_basic!(mgef, MagicEffect, values_magic_effect_flags:data:flags; id=effect_id=>EffectId::NightEye=>EffectId::DetectAnimal);
        test_flags!(mgef, MagicEffect, values_magic_effect_flags:data:flags; id=effect_id=>EffectId::RestoreFatigue=>EffectId::FortifyHealth);
        test_log_flags!(
            mgef,
            MagicEffect,
            false,
            "Merged MGEF record: Light\n",
            "Merging MGEF record: Light [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"data.flags\": - TARGET_ATTRIBUTE [\"Plugin1.esp\"]\n\"data.flags\": - NO_DURATION [\"Plugin1.esp\"]\n\"data.flags\": + NO_MAGNITUDE [\"Plugin1.esp\"]\nMerged MGEF record: Light\n",
            values_magic_effect_flags:data:flags,
            (effect_id = EffectId::Light)
        );
    }

    mod data_color {
        use super::{assert_eq, *};
        test_basic!(mgef, MagicEffect, values_magic_effect_color:data:color; id=effect_id=>EffectId::RallyHumanoid=>EffectId::DisintegrateArmor);
    }

    mod log {
        use super::{assert_eq, *};
        test_log!(
            mgef,
            MagicEffect,
            "Merged MGEF record: ResistNormalWeapons\n",
            "Merging MGEF record: ResistNormalWeapons [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"data.size_cap\": 0.0 -> 1.1 [\"Plugin1.esp\"]\nMerged MGEF record: ResistNormalWeapons\n",
            values_f32:data:size_cap,
            (effect_id = EffectId::ResistNormalWeapons)
        );
    }

    mod debug {
        use super::{assert_eq, *};
        test_debug!(
            mgef,
            MagicEffect,
            "Comparing to the last instance of MGEF record: SummonGhost [\"Plugin2.esp\"]\n\"texture\": \"string_1\" != \"\"\nMerged MGEF record: SummonGhost",
            values_string:texture,
            (effect_id = EffectId::SummonGhost)
            ; name="SummonGhost"
        );
    }
}
