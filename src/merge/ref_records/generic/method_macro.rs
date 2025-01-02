macro_rules! generic_ref_record_method_inventory {
    ($ref_record:ident) => {
        impl<'a> $ref_record<'a> {
            fn get_low_sorted_inventory(&mut self) -> Vec<&(i32, String)> {
                self.low.inventory = self
                    .inventory
                    .iter()
                    .map(|inventory| inventory_to_lowercase(inventory))
                    .collect::<Vec<_>>();
                let mut res = self.low.inventory.iter().collect::<Vec<_>>();
                res.sort();
                res
            }
        }
    };
}

macro_rules! generic_ref_record_method_spells {
    ($ref_record:ident) => {
        impl<'a> $ref_record<'a> {
            fn get_low_sorted_spells(&mut self) -> Vec<&String> {
                self.low.spells = self.spells.iter().map(|spell| spell_to_lowercase(spell)).collect::<Vec<_>>();
                let mut res = self.low.spells.iter().collect::<Vec<_>>();
                res.sort();
                res
            }
        }
    };
}

macro_rules! generic_ref_record_method_travel_destinations {
    ($ref_record:ident) => {
        impl<'a> $ref_record<'a> {
            fn get_low_sorted_travel_destinations(&mut self) -> Vec<&TravelDestination> {
                self.low.travel_destinations = self
                    .travel_destinations
                    .iter()
                    .map(|travel_destination| travel_destination_to_lowercase(travel_destination))
                    .collect::<Vec<_>>();
                let mut res = self.low.travel_destinations.iter().collect::<Vec<_>>();
                sort_travel_destinations(&mut res);
                res
            }
        }
    };
}

macro_rules! generic_ref_record_methods {
    (
        ($ref_record:ident, $tes3_record:ident$(, $id:ident)?),
        ($($field:ident),+),
        ($($clone_field:ident),*),
        ($($low:ident)?),
        ($($vec_field:ident),*),
        ($($ai_packages:ident)?)
    ) => {
        impl<'a> $ref_record<'a> {
            pub(super) fn new(source: &'a $tes3_record) -> $ref_record<'a> {
                $ref_record {
                    $($id: &source.$id,)?
                    $($clone_field: source.$clone_field.clone(),)*
                    $($vec_field: Vec::new(),)*
                    $(
                        base: source,
                        low: $low::default(),
                    )?
                    $($ai_packages: Vec::new(),)?
                    $($field: &source.$field,)+
                }
            }

            pub(super) fn into_owned(self) -> $tes3_record {
                $tes3_record {
                    $($id: self.$id.to_owned(),)?
                    $($clone_field: self.$clone_field,)*
                    $($vec_field: self.$vec_field,)*
                    $($ai_packages: self.$ai_packages,)?
                    $($field: self.$field.to_owned(),)+
                }
            }

            paste! {
                pub(super) fn equal(&mut self, target: &'a $tes3_record$(, $vec_field: &[<$vec_field:camel Helper>])*) -> Result<bool> {
                    #[allow(unused_mut)]
                    let mut res = true
                            $(&& self.$clone_field == target.$clone_field)*
                            $(&& *self.$field == target.$field)+;
                    $(
                        if res {
                            res = if self.$vec_field.len() == target.$vec_field.len() {
                                if *self.$vec_field != target.$vec_field {
                                    self.[<get_low_sorted_ $vec_field>]() == $vec_field.get_low_sorted_last()?
                                } else { true }
                            } else { false }
                        };
                    )*
                    $(
                        if res {
                                res = if self.$ai_packages.len() == target.$ai_packages.len() {
                                    if *self.$ai_packages != target.$ai_packages {
                                        if self.$ai_packages.len() == 1 {
                                            ai_packages_equal(&self.$ai_packages[0], &target.$ai_packages[0])?
                                        } else { false }
                                    } else { true }
                                } else { false }
                        };
                    )?
                    Ok(res)
                }
            }
        }
    }
}

pub(crate) use {
    generic_ref_record_method_inventory, generic_ref_record_method_spells, generic_ref_record_method_travel_destinations,
    generic_ref_record_methods,
};
