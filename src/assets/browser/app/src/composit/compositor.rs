use composit::{
    ActiveSource, Stick, Scale, ComponentSet, StateManager, Stage
};

use model::driver::{ PrinterManager, Printer };
use model::train::{ Train, TrainManager, TravellerCreator };
use drivers::zmenu::{ ZMenuRegistry, ZMenuLeafSet };

use controller::global::AppRunner;
use controller::input::Action;
use controller::output::Report;
use data::{ Psychic, PsychicPacer, XferCache, XferClerk };
use types::{ DOWN, Dot };

const MS_PER_UPDATE : f64 = 0.;
const MS_PRIME_DELAY: f64 = 2000.;

pub struct Compositor {
    train_manager: TrainManager,
    zmr: ZMenuRegistry,
    bp_per_screen: f64,
    updated: bool,
    prime_delay: Option<f64>,
    last_updated: Option<f64>,
    components: TravellerCreator,
    wanted_componentset: ComponentSet,
    current_componentset: ComponentSet,
    psychic: Psychic,
    pacer: PsychicPacer,
    xfercache: XferCache,
    xferclerk: Box<XferClerk>
}

impl Compositor {
    pub fn new(printer: PrinterManager, xfercache: &XferCache, xferclerk: Box<XferClerk>) -> Compositor {
        Compositor {
            train_manager: TrainManager::new(&printer),
            zmr: ZMenuRegistry::new(),
            components: TravellerCreator::new(&printer),
            bp_per_screen: 1.,
            updated: true,
            last_updated: None,
            wanted_componentset: ComponentSet::new(),
            current_componentset: ComponentSet::new(),
            psychic: Psychic::new(),
            prime_delay: None,
            pacer: PsychicPacer::new(30000.),
            xfercache: xfercache.clone(),
            xferclerk: xferclerk
        }
    }

    pub fn get_zmr(&self) -> &ZMenuRegistry { &self.zmr }

    pub fn get_prop_trans(&self) -> f32 { self.train_manager.get_prop_trans() }

    fn prime_cache(&mut self, t: f64) {
        if self.prime_delay.is_none() {
            self.prime_delay = Some(t);
        }
        if t - self.prime_delay.unwrap() > MS_PRIME_DELAY {
            if let Some(leafs) = self.psychic.get_leafs() {
                let leafs = self.pacer.filter_leafs(leafs,t);
                for leaf in leafs.iter() {
                    for comp in self.wanted_componentset.list_names() {
                        self.xfercache.prime(&mut self.xferclerk,&comp,&leaf);
                    }
                }
            }
        }
    }

    pub fn tick(&mut self, t: f64) {
        /* Manage components */
        for added in self.wanted_componentset.not_in(&self.current_componentset).iter() {
            self.add_component(added.clone());
        }
        for removed in self.current_componentset.not_in(&self.wanted_componentset).iter() {
            self.components.remove_source(removed.clone().get_name());
        }
        self.current_componentset = self.wanted_componentset.clone();
        /* Warm up xfercache */
        self.prime_cache(t);
        /* Move into future */
        self.train_manager.tick(t,&mut self.components);
        /* Manage useful leafs */
        if self.updated {
            if let Some(prev_t) = self.last_updated {
                if t-prev_t < MS_PER_UPDATE { return; }
            }
            let comps = &mut self.components;
            self.train_manager.best_train(|sc| sc.manage_leafs(comps));            
            self.updated = false;
            self.last_updated = Some(t);
        }
    }

    pub fn update_report(&self, report: &Report) {
        self.train_manager.update_report(report);
    }

    pub fn set_stick(&mut self, st: &Stick) {
        self.prime_delay = None; // Force priming delay as screen is completely invalid
        self.train_manager.set_stick(st,self.bp_per_screen);
        self.psychic.set_stick(st);
        self.updated = true;
    }

    pub fn set_position(&mut self, position_bp: f64) {
        self.train_manager.set_position(position_bp);
        self.psychic.set_middle(position_bp as i64);
        self.updated = true;
    }
    
    pub fn set_zoom(&mut self, bp_per_screen: f64) {
        self.bp_per_screen = bp_per_screen;
        self.train_manager.set_zoom(&mut self.components, bp_per_screen);
        self.psychic.set_scale(&Scale::best_for_screen(bp_per_screen));
        self.psychic.set_width(bp_per_screen as i32);
        self.updated = true;
    }

    pub fn get_current_train(&mut self) -> Option<&mut Train> {
        self.train_manager.get_current_train()
    }

    pub fn get_transition_train(&mut self) -> Option<&mut Train> {
        self.train_manager.get_transition_train()
    }
    
    pub fn get_component_set(&mut self) -> &mut ComponentSet {
        &mut self.wanted_componentset
    }
    
    pub fn redraw_where_needed(&mut self, printer: &mut Printer) {
        let mut zmls = ZMenuLeafSet::new();
        if let Some(train) = self.get_current_train() {
            train.redraw_where_needed(printer,&mut zmls);
        }
        if let Some(train) = self.get_transition_train() {
            train.redraw_where_needed(printer,&mut zmls);
        }
        self.zmr.add_leafset(zmls);
    }

    fn add_component(&mut self, mut c: ActiveSource) {
        {
            let cc = &mut self.components;
            self.train_manager.each_train(|sc|
                sc.add_component(cc,&mut c)
            );
        }
        self.components.add_source(c);
    }
    
    pub fn update_state(&mut self, oom: &StateManager) {
        self.train_manager.update_state(oom);
    }
    
    pub fn intersects(&self, stage: &Stage, pos: Dot<i32,i32>) -> Vec<Action> {
        self.zmr.intersects(stage,pos)
    }
}

pub fn register_compositor_ticks(ar: &mut AppRunner) {
    ar.add_timer("compositor",|cs,t,_| {
        cs.with_compo(|co| co.tick(t) );
        let max_y = cs.get_all_landscapes().get_low_watermark();
        cs.with_stage(|s| s.set_limit(&DOWN,max_y as f64));
        vec!{}
    },2);
}
