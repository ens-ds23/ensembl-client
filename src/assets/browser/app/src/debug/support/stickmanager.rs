/* In the test-cards, every testcard is a separate stick */

use composit::{ CombinedStickManager, Stick };

pub fn add_debug_sticks(csm: &mut CombinedStickManager) {
    csm.add_internal_stick("polar",Stick::new("polar",17000000,false));
    csm.add_internal_stick("march",Stick::new("march",17000000,false));
    csm.add_internal_stick("text",Stick::new("text",17000000,false));
    csm.add_internal_stick("leaf",Stick::new("leaf",17000000,false));
    csm.add_internal_stick("ruler",Stick::new("ruler",17000000,false));
    csm.add_internal_stick("button",Stick::new("button",17000000,false));
    csm.add_internal_stick("tácode",Stick::new("tácode",17000000,false));
}
