use super::opcodes::Instruction;

impl Instruction {
    pub fn size(&self) -> usize {
        match *self {
            Instruction::NOP => 1,
            Instruction::LD_BC_d16(_) => 3,
            Instruction::LD_pBC_A => 1,
            Instruction::INC_BC => 1,
            Instruction::INC_B => 1,
            Instruction::DEC_B => 1,
            Instruction::LD_B_d8(_) => 2,
            Instruction::RLCA => 1,
            Instruction::LD_pa16_SP(_) => 3,
            Instruction::ADD_HL_BC => 1,
            Instruction::LD_A_pBC => 1,
            Instruction::DEC_BC => 1,
            Instruction::INC_C => 1,
            Instruction::DEC_C => 1,
            Instruction::LD_C_d8(_) => 2,
            Instruction::RRCA => 1,
            Instruction::STOP_0 => 1,
            Instruction::LD_DE_d16(_) => 3,
            Instruction::LD_pDE_A => 1,
            Instruction::INC_DE => 1,
            Instruction::INC_D => 1,
            Instruction::DEC_D => 1,
            Instruction::LD_D_d8(_) => 2,
            Instruction::RLA => 1,
            Instruction::JR_r8(_) => 2,
            Instruction::ADD_HL_DE => 1,
            Instruction::LD_A_pDE => 1,
            Instruction::DEC_DE => 1,
            Instruction::INC_E => 1,
            Instruction::DEC_E => 1,
            Instruction::LD_E_d8(_) => 2,
            Instruction::RRA => 1,
            Instruction::JR_NZ_r8(_) => 2,
            Instruction::LD_HL_d16(_) => 3,
            Instruction::LD_pHLp_A => 1,
            Instruction::INC_HL => 1,
            Instruction::INC_H => 1,
            Instruction::DEC_H => 1,
            Instruction::LD_H_d8(_) => 2,
            Instruction::DAA => 1,
            Instruction::JR_Z_r8(_) => 2,
            Instruction::ADD_HL_HL => 1,
            Instruction::LD_A_pHLp => 1,
            Instruction::DEC_HL => 1,
            Instruction::INC_L => 1,
            Instruction::DEC_L => 1,
            Instruction::LD_L_d8(_) => 2,
            Instruction::CPL => 1,
            Instruction::JR_NC_r8(_) => 2,
            Instruction::LD_SP_d16(_) => 3,
            Instruction::LD_pHLm_A => 1,
            Instruction::INC_SP => 1,
            Instruction::INC_pHL => 1,
            Instruction::DEC_pHL => 1,
            Instruction::LD_pHL_d8(_) => 2,
            Instruction::SCF => 1,
            Instruction::JR_C_r8(_) => 2,
            Instruction::ADD_HL_SP => 1,
            Instruction::LD_A_pHLm => 1,
            Instruction::DEC_SP => 1,
            Instruction::INC_A => 1,
            Instruction::DEC_A => 1,
            Instruction::LD_A_d8(_) => 2,
            Instruction::CCF => 1,
            Instruction::LD_B_B => 1,
            Instruction::LD_B_C => 1,
            Instruction::LD_B_D => 1,
            Instruction::LD_B_E => 1,
            Instruction::LD_B_H => 1,
            Instruction::LD_B_L => 1,
            Instruction::LD_B_pHL => 1,
            Instruction::LD_B_A => 1,
            Instruction::LD_C_B => 1,
            Instruction::LD_C_C => 1,
            Instruction::LD_C_D => 1,
            Instruction::LD_C_E => 1,
            Instruction::LD_C_H => 1,
            Instruction::LD_C_L => 1,
            Instruction::LD_C_pHL => 1,
            Instruction::LD_C_A => 1,
            Instruction::LD_D_B => 1,
            Instruction::LD_D_C => 1,
            Instruction::LD_D_D => 1,
            Instruction::LD_D_E => 1,
            Instruction::LD_D_H => 1,
            Instruction::LD_D_L => 1,
            Instruction::LD_D_pHL => 1,
            Instruction::LD_D_A => 1,
            Instruction::LD_E_B => 1,
            Instruction::LD_E_C => 1,
            Instruction::LD_E_D => 1,
            Instruction::LD_E_E => 1,
            Instruction::LD_E_H => 1,
            Instruction::LD_E_L => 1,
            Instruction::LD_E_pHL => 1,
            Instruction::LD_E_A => 1,
            Instruction::LD_H_B => 1,
            Instruction::LD_H_C => 1,
            Instruction::LD_H_D => 1,
            Instruction::LD_H_E => 1,
            Instruction::LD_H_H => 1,
            Instruction::LD_H_L => 1,
            Instruction::LD_H_pHL => 1,
            Instruction::LD_H_A => 1,
            Instruction::LD_L_B => 1,
            Instruction::LD_L_C => 1,
            Instruction::LD_L_D => 1,
            Instruction::LD_L_E => 1,
            Instruction::LD_L_H => 1,
            Instruction::LD_L_L => 1,
            Instruction::LD_L_pHL => 1,
            Instruction::LD_L_A => 1,
            Instruction::LD_pHL_B => 1,
            Instruction::LD_pHL_C => 1,
            Instruction::LD_pHL_D => 1,
            Instruction::LD_pHL_E => 1,
            Instruction::LD_pHL_H => 1,
            Instruction::LD_pHL_L => 1,
            Instruction::HALT => 1,
            Instruction::LD_pHL_A => 1,
            Instruction::LD_A_B => 1,
            Instruction::LD_A_C => 1,
            Instruction::LD_A_D => 1,
            Instruction::LD_A_E => 1,
            Instruction::LD_A_H => 1,
            Instruction::LD_A_L => 1,
            Instruction::LD_A_pHL => 1,
            Instruction::LD_A_A => 1,
            Instruction::ADD_A_B => 1,
            Instruction::ADD_A_C => 1,
            Instruction::ADD_A_D => 1,
            Instruction::ADD_A_E => 1,
            Instruction::ADD_A_H => 1,
            Instruction::ADD_A_L => 1,
            Instruction::ADD_A_pHL => 1,
            Instruction::ADD_A_A => 1,
            Instruction::ADC_A_B => 1,
            Instruction::ADC_A_C => 1,
            Instruction::ADC_A_D => 1,
            Instruction::ADC_A_E => 1,
            Instruction::ADC_A_H => 1,
            Instruction::ADC_A_L => 1,
            Instruction::ADC_A_pHL => 1,
            Instruction::ADC_A_A => 1,
            Instruction::SUB_B => 1,
            Instruction::SUB_C => 1,
            Instruction::SUB_D => 1,
            Instruction::SUB_E => 1,
            Instruction::SUB_H => 1,
            Instruction::SUB_L => 1,
            Instruction::SUB_pHL => 1,
            Instruction::SUB_A => 1,
            Instruction::SBC_A_B => 1,
            Instruction::SBC_A_C => 1,
            Instruction::SBC_A_D => 1,
            Instruction::SBC_A_E => 1,
            Instruction::SBC_A_H => 1,
            Instruction::SBC_A_L => 1,
            Instruction::SBC_A_pHL => 1,
            Instruction::SBC_A_A => 1,
            Instruction::AND_B => 1,
            Instruction::AND_C => 1,
            Instruction::AND_D => 1,
            Instruction::AND_E => 1,
            Instruction::AND_H => 1,
            Instruction::AND_L => 1,
            Instruction::AND_pHL => 1,
            Instruction::AND_A => 1,
            Instruction::XOR_B => 1,
            Instruction::XOR_C => 1,
            Instruction::XOR_D => 1,
            Instruction::XOR_E => 1,
            Instruction::XOR_H => 1,
            Instruction::XOR_L => 1,
            Instruction::XOR_pHL => 1,
            Instruction::XOR_A => 1,
            Instruction::OR_B => 1,
            Instruction::OR_C => 1,
            Instruction::OR_D => 1,
            Instruction::OR_E => 1,
            Instruction::OR_H => 1,
            Instruction::OR_L => 1,
            Instruction::OR_pHL => 1,
            Instruction::OR_A => 1,
            Instruction::CP_B => 1,
            Instruction::CP_C => 1,
            Instruction::CP_D => 1,
            Instruction::CP_E => 1,
            Instruction::CP_H => 1,
            Instruction::CP_L => 1,
            Instruction::CP_pHL => 1,
            Instruction::CP_A => 1,
            Instruction::RET_NZ => 1,
            Instruction::POP_BC => 1,
            Instruction::JP_NZ_a16(_) => 3,
            Instruction::JP_a16(_) => 3,
            Instruction::CALL_NZ_a16(_) => 3,
            Instruction::PUSH_BC => 1,
            Instruction::ADD_A_d8(_) => 2,
            Instruction::RST_00H => 1,
            Instruction::RET_Z => 1,
            Instruction::RET => 1,
            Instruction::JP_Z_a16(_) => 3,
            Instruction::PREFIX_CB(_) => 2,
            Instruction::CALL_Z_a16(_) => 3,
            Instruction::CALL_a16(_) => 3,
            Instruction::ADC_A_d8(_) => 2,
            Instruction::RST_08H => 1,
            Instruction::RET_NC => 1,
            Instruction::POP_DE => 1,
            Instruction::JP_NC_a16(_) => 3,
            Instruction::CALL_NC_a16(_) => 3,
            Instruction::PUSH_DE => 1,
            Instruction::SUB_d8(_) => 2,
            Instruction::RST_10H => 1,
            Instruction::RET_C => 1,
            Instruction::RETI => 1,
            Instruction::JP_C_a16(_) => 3,
            Instruction::CALL_C_a16(_) => 3,
            Instruction::SBC_A_d8(_) => 2,
            Instruction::RST_18H => 1,
            Instruction::LDH_pa8_A(_) => 2,
            Instruction::POP_HL => 1,
            Instruction::LD_pC_A => 1,
            Instruction::PUSH_HL => 1,
            Instruction::AND_d8(_) => 2,
            Instruction::RST_20H => 1,
            Instruction::ADD_SP_r8(_) => 2,
            Instruction::JP_pHL => 1,
            Instruction::LD_pa16_A(_) => 3,
            Instruction::XOR_d8(_) => 2,
            Instruction::RST_28H => 1,
            Instruction::LDH_A_pa8(_) => 2,
            Instruction::POP_AF => 1,
            Instruction::LD_A_pC => 1,
            Instruction::DI => 1,
            Instruction::PUSH_AF => 1,
            Instruction::OR_d8(_) => 2,
            Instruction::RST_30H => 1,
            Instruction::LD_HL_SPp_r8(_) => 2,
            Instruction::LD_SP_HL => 1,
            Instruction::LD_A_pa16(_) => 3,
            Instruction::EI => 1,
            Instruction::CP_d8(_) => 2,
            Instruction::RST_38H => 1
        }
    }
}
