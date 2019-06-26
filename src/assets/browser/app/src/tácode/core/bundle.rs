use tánaiste::{ Instruction, InstructionBundle };

use super::super::core::TáContext;
use super::super::commands::{
    AppGetI, CPrintI, DPrintI, AbuttI, ElideI, NotI, PickI, ZTmplSpecI,
    ExtentI, AllI, BinOpI, BinOpType, TextI, IndexI, RunsI, RunsOfI,
    GetI, ScaleI, MergeI, AccNI, MemberI, PlotI, RulerI, AllPlotsI,
    LengthI, SetPartI, ImageI, AssetI, Text2I, LengthsI, BurstI, ZTmplI,
    ZMenuI
};
use super::super::shapecmd::ShapeI;

pub fn instruction_bundle_app(tc: &TáContext) -> InstructionBundle {
    let mut ib = InstructionBundle::new();
    ib.add_all(vec! {
        Box::new(CPrintI()) as Box<Instruction>,
        Box::new(DPrintI()),
        Box::new(ElideI()),
        Box::new(AbuttI()),
        Box::new(NotI()),
        Box::new(AllI()),
        Box::new(PickI()),
        Box::new(IndexI()),
        Box::new(RunsI()),
        Box::new(RunsOfI()),
        Box::new(GetI()),
        Box::new(MergeI()),
        Box::new(AccNI()),
        Box::new(MemberI()),
        Box::new(LengthI()),
        Box::new(LengthsI()),
        Box::new(BurstI()),
        Box::new(BinOpI(BinOpType::Add)),
        Box::new(BinOpI(BinOpType::Mul)),
        Box::new(BinOpI(BinOpType::Div)),
        Box::new(BinOpI(BinOpType::Eq)),
        Box::new(BinOpI(BinOpType::Or)),
        Box::new(BinOpI(BinOpType::Max)),
        Box::new(BinOpI(BinOpType::Min)),
        Box::new(ImageI(tc.clone())),
        Box::new(TextI(tc.clone())),
        Box::new(Text2I(tc.clone())),
        Box::new(AssetI(tc.clone())),
        Box::new(AppGetI(tc.clone())),
        Box::new(ShapeI(tc.clone())),
        Box::new(ExtentI(tc.clone())),
        Box::new(ScaleI(tc.clone())),
        Box::new(PlotI(tc.clone())),
        Box::new(RulerI(tc.clone())),
        Box::new(AllPlotsI(tc.clone())),
        Box::new(SetPartI(tc.clone())),
        Box::new(ZTmplSpecI(tc.clone())),
        Box::new(ZTmplI(tc.clone())),
        Box::new(ZMenuI(tc.clone())),
    });
    ib
}
