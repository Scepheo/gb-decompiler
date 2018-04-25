use std::fmt;
use super::opcodes::Instruction;
use super::opcodes::CBInstruction;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::LD_BC_d16(value) => write!(f, "LD BC,{}", value),
            Instruction::LD_pBC_A => write!(f, "LD (BC),A"),
            Instruction::INC_BC => write!(f, "INC BC"),
            Instruction::INC_B => write!(f, "INC B"),
            Instruction::DEC_B => write!(f, "DEC B"),
            Instruction::LD_B_d8(value) => write!(f, "LD B,{}", value),
            Instruction::RLCA => write!(f, "RLCA"),
            Instruction::LD_pa16_SP(value) => write!(f, "LD ({}),SP", value),
            Instruction::ADD_HL_BC => write!(f, "ADD HL,BC"),
            Instruction::LD_A_pBC => write!(f, "LD A,(BC)"),
            Instruction::DEC_BC => write!(f, "DEC BC"),
            Instruction::INC_C => write!(f, "INC C"),
            Instruction::DEC_C => write!(f, "DEC C"),
            Instruction::LD_C_d8(value) => write!(f, "LD C,{}", value),
            Instruction::RRCA => write!(f, "RRCA"),
            Instruction::STOP_0 => write!(f, "STOP 0"),
            Instruction::LD_DE_d16(value) => write!(f, "LD DE,{}", value),
            Instruction::LD_pDE_A => write!(f, "LD (DE),A"),
            Instruction::INC_DE => write!(f, "INC DE"),
            Instruction::INC_D => write!(f, "INC D"),
            Instruction::DEC_D => write!(f, "DEC D"),
            Instruction::LD_D_d8(value) => write!(f, "LD D,{}", value),
            Instruction::RLA => write!(f, "RLA"),
            Instruction::JR_r8(value) => write!(f, "JR {}", value),
            Instruction::ADD_HL_DE => write!(f, "ADD HL,DE"),
            Instruction::LD_A_pDE => write!(f, "LD A,(DE)"),
            Instruction::DEC_DE => write!(f, "DEC DE"),
            Instruction::INC_E => write!(f, "INC E"),
            Instruction::DEC_E => write!(f, "DEC E"),
            Instruction::LD_E_d8(value) => write!(f, "LD E,{}", value),
            Instruction::RRA => write!(f, "RRA"),
            Instruction::JR_NZ_r8(value) => write!(f, "JR NZ,{}", value),
            Instruction::LD_HL_d16(value) => write!(f, "LD HL,{}", value),
            Instruction::LD_pHLp_A => write!(f, "LD (HL+),A"),
            Instruction::INC_HL => write!(f, "INC HL"),
            Instruction::INC_H => write!(f, "INC H"),
            Instruction::DEC_H => write!(f, "DEC H"),
            Instruction::LD_H_d8(value) => write!(f, "LD H,{}", value),
            Instruction::DAA => write!(f, "DAA"),
            Instruction::JR_Z_r8(value) => write!(f, "JR Z,{}", value),
            Instruction::ADD_HL_HL => write!(f, "ADD HL,HL"),
            Instruction::LD_A_pHLp => write!(f, "LD A,(HL+)"),
            Instruction::DEC_HL => write!(f, "DEC HL"),
            Instruction::INC_L => write!(f, "INC L"),
            Instruction::DEC_L => write!(f, "DEC L"),
            Instruction::LD_L_d8(value) => write!(f, "LD L,{}", value),
            Instruction::CPL => write!(f, "CPL"),
            Instruction::JR_NC_r8(value) => write!(f, "JR NC,{}", value),
            Instruction::LD_SP_d16(value) => write!(f, "LD SP,{}", value),
            Instruction::LD_pHLm_A => write!(f, "LD (HL-),A"),
            Instruction::INC_SP => write!(f, "INC SP"),
            Instruction::INC_pHL => write!(f, "INC (HL)"),
            Instruction::DEC_pHL => write!(f, "DEC (HL)"),
            Instruction::LD_pHL_d8(value) => write!(f, "LD (HL),{}", value),
            Instruction::SCF => write!(f, "SCF"),
            Instruction::JR_C_r8(value) => write!(f, "JR C,{}", value),
            Instruction::ADD_HL_SP => write!(f, "ADD HL,SP"),
            Instruction::LD_A_pHLm => write!(f, "LD A,(HL-)"),
            Instruction::DEC_SP => write!(f, "DEC SP"),
            Instruction::INC_A => write!(f, "INC A"),
            Instruction::DEC_A => write!(f, "DEC A"),
            Instruction::LD_A_d8(value) => write!(f, "LD A,{}", value),
            Instruction::CCF => write!(f, "CCF"),
            Instruction::LD_B_B => write!(f, "LD B,B"),
            Instruction::LD_B_C => write!(f, "LD B,C"),
            Instruction::LD_B_D => write!(f, "LD B,D"),
            Instruction::LD_B_E => write!(f, "LD B,E"),
            Instruction::LD_B_H => write!(f, "LD B,H"),
            Instruction::LD_B_L => write!(f, "LD B,L"),
            Instruction::LD_B_pHL => write!(f, "LD B,(HL)"),
            Instruction::LD_B_A => write!(f, "LD B,A"),
            Instruction::LD_C_B => write!(f, "LD C,B"),
            Instruction::LD_C_C => write!(f, "LD C,C"),
            Instruction::LD_C_D => write!(f, "LD C,D"),
            Instruction::LD_C_E => write!(f, "LD C,E"),
            Instruction::LD_C_H => write!(f, "LD C,H"),
            Instruction::LD_C_L => write!(f, "LD C,L"),
            Instruction::LD_C_pHL => write!(f, "LD C,(HL)"),
            Instruction::LD_C_A => write!(f, "LD C,A"),
            Instruction::LD_D_B => write!(f, "LD D,B"),
            Instruction::LD_D_C => write!(f, "LD D,C"),
            Instruction::LD_D_D => write!(f, "LD D,D"),
            Instruction::LD_D_E => write!(f, "LD D,E"),
            Instruction::LD_D_H => write!(f, "LD D,H"),
            Instruction::LD_D_L => write!(f, "LD D,L"),
            Instruction::LD_D_pHL => write!(f, "LD D,(HL)"),
            Instruction::LD_D_A => write!(f, "LD D,A"),
            Instruction::LD_E_B => write!(f, "LD E,B"),
            Instruction::LD_E_C => write!(f, "LD E,C"),
            Instruction::LD_E_D => write!(f, "LD E,D"),
            Instruction::LD_E_E => write!(f, "LD E,E"),
            Instruction::LD_E_H => write!(f, "LD E,H"),
            Instruction::LD_E_L => write!(f, "LD E,L"),
            Instruction::LD_E_pHL => write!(f, "LD E,(HL)"),
            Instruction::LD_E_A => write!(f, "LD E,A"),
            Instruction::LD_H_B => write!(f, "LD H,B"),
            Instruction::LD_H_C => write!(f, "LD H,C"),
            Instruction::LD_H_D => write!(f, "LD H,D"),
            Instruction::LD_H_E => write!(f, "LD H,E"),
            Instruction::LD_H_H => write!(f, "LD H,H"),
            Instruction::LD_H_L => write!(f, "LD H,L"),
            Instruction::LD_H_pHL => write!(f, "LD H,(HL)"),
            Instruction::LD_H_A => write!(f, "LD H,A"),
            Instruction::LD_L_B => write!(f, "LD L,B"),
            Instruction::LD_L_C => write!(f, "LD L,C"),
            Instruction::LD_L_D => write!(f, "LD L,D"),
            Instruction::LD_L_E => write!(f, "LD L,E"),
            Instruction::LD_L_H => write!(f, "LD L,H"),
            Instruction::LD_L_L => write!(f, "LD L,L"),
            Instruction::LD_L_pHL => write!(f, "LD L,(HL)"),
            Instruction::LD_L_A => write!(f, "LD L,A"),
            Instruction::LD_pHL_B => write!(f, "LD (HL),B"),
            Instruction::LD_pHL_C => write!(f, "LD (HL),C"),
            Instruction::LD_pHL_D => write!(f, "LD (HL),D"),
            Instruction::LD_pHL_E => write!(f, "LD (HL),E"),
            Instruction::LD_pHL_H => write!(f, "LD (HL),H"),
            Instruction::LD_pHL_L => write!(f, "LD (HL),L"),
            Instruction::HALT => write!(f, "HALT"),
            Instruction::LD_pHL_A => write!(f, "LD (HL),A"),
            Instruction::LD_A_B => write!(f, "LD A,B"),
            Instruction::LD_A_C => write!(f, "LD A,C"),
            Instruction::LD_A_D => write!(f, "LD A,D"),
            Instruction::LD_A_E => write!(f, "LD A,E"),
            Instruction::LD_A_H => write!(f, "LD A,H"),
            Instruction::LD_A_L => write!(f, "LD A,L"),
            Instruction::LD_A_pHL => write!(f, "LD A,(HL)"),
            Instruction::LD_A_A => write!(f, "LD A,A"),
            Instruction::ADD_A_B => write!(f, "ADD A,B"),
            Instruction::ADD_A_C => write!(f, "ADD A,C"),
            Instruction::ADD_A_D => write!(f, "ADD A,D"),
            Instruction::ADD_A_E => write!(f, "ADD A,E"),
            Instruction::ADD_A_H => write!(f, "ADD A,H"),
            Instruction::ADD_A_L => write!(f, "ADD A,L"),
            Instruction::ADD_A_pHL => write!(f, "ADD A,(HL)"),
            Instruction::ADD_A_A => write!(f, "ADD A,A"),
            Instruction::ADC_A_B => write!(f, "ADC A,B"),
            Instruction::ADC_A_C => write!(f, "ADC A,C"),
            Instruction::ADC_A_D => write!(f, "ADC A,D"),
            Instruction::ADC_A_E => write!(f, "ADC A,E"),
            Instruction::ADC_A_H => write!(f, "ADC A,H"),
            Instruction::ADC_A_L => write!(f, "ADC A,L"),
            Instruction::ADC_A_pHL => write!(f, "ADC A,(HL)"),
            Instruction::ADC_A_A => write!(f, "ADC A,A"),
            Instruction::SUB_B => write!(f, "SUB B"),
            Instruction::SUB_C => write!(f, "SUB C"),
            Instruction::SUB_D => write!(f, "SUB D"),
            Instruction::SUB_E => write!(f, "SUB E"),
            Instruction::SUB_H => write!(f, "SUB H"),
            Instruction::SUB_L => write!(f, "SUB L"),
            Instruction::SUB_pHL => write!(f, "SUB (HL)"),
            Instruction::SUB_A => write!(f, "SUB A"),
            Instruction::SBC_A_B => write!(f, "SBC A,B"),
            Instruction::SBC_A_C => write!(f, "SBC A,C"),
            Instruction::SBC_A_D => write!(f, "SBC A,D"),
            Instruction::SBC_A_E => write!(f, "SBC A,E"),
            Instruction::SBC_A_H => write!(f, "SBC A,H"),
            Instruction::SBC_A_L => write!(f, "SBC A,L"),
            Instruction::SBC_A_pHL => write!(f, "SBC A,(HL)"),
            Instruction::SBC_A_A => write!(f, "SBC A,A"),
            Instruction::AND_B => write!(f, "AND B"),
            Instruction::AND_C => write!(f, "AND C"),
            Instruction::AND_D => write!(f, "AND D"),
            Instruction::AND_E => write!(f, "AND E"),
            Instruction::AND_H => write!(f, "AND H"),
            Instruction::AND_L => write!(f, "AND L"),
            Instruction::AND_pHL => write!(f, "AND (HL)"),
            Instruction::AND_A => write!(f, "AND A"),
            Instruction::XOR_B => write!(f, "XOR B"),
            Instruction::XOR_C => write!(f, "XOR C"),
            Instruction::XOR_D => write!(f, "XOR D"),
            Instruction::XOR_E => write!(f, "XOR E"),
            Instruction::XOR_H => write!(f, "XOR H"),
            Instruction::XOR_L => write!(f, "XOR L"),
            Instruction::XOR_pHL => write!(f, "XOR (HL)"),
            Instruction::XOR_A => write!(f, "XOR A"),
            Instruction::OR_B => write!(f, "OR B"),
            Instruction::OR_C => write!(f, "OR C"),
            Instruction::OR_D => write!(f, "OR D"),
            Instruction::OR_E => write!(f, "OR E"),
            Instruction::OR_H => write!(f, "OR H"),
            Instruction::OR_L => write!(f, "OR L"),
            Instruction::OR_pHL => write!(f, "OR (HL)"),
            Instruction::OR_A => write!(f, "OR A"),
            Instruction::CP_B => write!(f, "CP B"),
            Instruction::CP_C => write!(f, "CP C"),
            Instruction::CP_D => write!(f, "CP D"),
            Instruction::CP_E => write!(f, "CP E"),
            Instruction::CP_H => write!(f, "CP H"),
            Instruction::CP_L => write!(f, "CP L"),
            Instruction::CP_pHL => write!(f, "CP (HL)"),
            Instruction::CP_A => write!(f, "CP A"),
            Instruction::RET_NZ => write!(f, "RET NZ"),
            Instruction::POP_BC => write!(f, "POP BC"),
            Instruction::JP_NZ_a16(value) => write!(f, "JP NZ,{}", value),
            Instruction::JP_a16(value) => write!(f, "JP {}", value),
            Instruction::CALL_NZ_a16(value) => write!(f, "CALL NZ,{}", value),
            Instruction::PUSH_BC => write!(f, "PUSH BC"),
            Instruction::ADD_A_d8(value) => write!(f, "ADD A,{}", value),
            Instruction::RST_00H => write!(f, "RST 00H"),
            Instruction::RET_Z => write!(f, "RET Z"),
            Instruction::RET => write!(f, "RET"),
            Instruction::JP_Z_a16(value) => write!(f, "JP Z,{}", value),
            Instruction::PREFIX_CB(value) => value.fmt(f),
            Instruction::CALL_Z_a16(value) => write!(f, "CALL Z,{}", value),
            Instruction::CALL_a16(value) => write!(f, "CALL {}", value),
            Instruction::ADC_A_d8(value) => write!(f, "ADC A,{}", value),
            Instruction::RST_08H => write!(f, "RST 08H"),
            Instruction::RET_NC => write!(f, "RET NC"),
            Instruction::POP_DE => write!(f, "POP DE"),
            Instruction::JP_NC_a16(value) => write!(f, "JP NC,{}", value),
            Instruction::CALL_NC_a16(value) => write!(f, "CALL NC,{}", value),
            Instruction::PUSH_DE => write!(f, "PUSH DE"),
            Instruction::SUB_d8(value) => write!(f, "SUB {}", value),
            Instruction::RST_10H => write!(f, "RST 10H"),
            Instruction::RET_C => write!(f, "RET C"),
            Instruction::RETI => write!(f, "RETI"),
            Instruction::JP_C_a16(value) => write!(f, "JP C,{}", value),
            Instruction::CALL_C_a16(value) => write!(f, "CALL C,{}", value),
            Instruction::SBC_A_d8(value) => write!(f, "SBC A,{}", value),
            Instruction::RST_18H => write!(f, "RST 18H"),
            Instruction::LDH_pa8_A(value) => write!(f, "LDH ({}),A", value),
            Instruction::POP_HL => write!(f, "POP HL"),
            Instruction::LD_pC_A => write!(f, "LD (C),A"),
            Instruction::PUSH_HL => write!(f, "PUSH HL"),
            Instruction::AND_d8(value) => write!(f, "AND {}", value),
            Instruction::RST_20H => write!(f, "RST 20H"),
            Instruction::ADD_SP_r8(value) => write!(f, "ADD SP,{}", value),
            Instruction::JP_pHL => write!(f, "JP (HL)"),
            Instruction::LD_pa16_A(value) => write!(f, "LD ({}),A", value),
            Instruction::XOR_d8(value) => write!(f, "XOR {}", value),
            Instruction::RST_28H => write!(f, "RST 28H"),
            Instruction::LDH_A_pa8(value) => write!(f, "LDH A,({})", value),
            Instruction::POP_AF => write!(f, "POP AF"),
            Instruction::LD_A_pC => write!(f, "LD A,(C)"),
            Instruction::DI => write!(f, "DI"),
            Instruction::PUSH_AF => write!(f, "PUSH AF"),
            Instruction::OR_d8(value) => write!(f, "OR {}", value),
            Instruction::RST_30H => write!(f, "RST 30H"),
            Instruction::LD_HL_SPp_r8(value) => write!(f, "LD HL,SP+{}", value),
            Instruction::LD_SP_HL => write!(f, "LD SP,HL"),
            Instruction::LD_A_pa16(value) => write!(f, "LD A,({})", value),
            Instruction::EI => write!(f, "EI"),
            Instruction::CP_d8(value) => write!(f, "CP {}", value),
            Instruction::RST_38H => write!(f, "RST 38H")
        }
    }
}

impl fmt::Display for CBInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CBInstruction::RLC_B => write!(f, "RLC B"),
            CBInstruction::RLC_C => write!(f, "RLC C"),
            CBInstruction::RLC_D => write!(f, "RLC D"),
            CBInstruction::RLC_E => write!(f, "RLC E"),
            CBInstruction::RLC_H => write!(f, "RLC H"),
            CBInstruction::RLC_L => write!(f, "RLC L"),
            CBInstruction::RLC_pHL => write!(f, "RLC (pHL)"),
            CBInstruction::RLC_A => write!(f, "RLC A"),
            CBInstruction::RRC_B => write!(f, "RRC B"),
            CBInstruction::RRC_C => write!(f, "RRC C"),
            CBInstruction::RRC_D => write!(f, "RRC D"),
            CBInstruction::RRC_E => write!(f, "RRC E"),
            CBInstruction::RRC_H => write!(f, "RRC H"),
            CBInstruction::RRC_L => write!(f, "RRC L"),
            CBInstruction::RRC_pHL => write!(f, "RRC (pHL)"),
            CBInstruction::RRC_A => write!(f, "RRC A"),
            CBInstruction::RL_B => write!(f, "RL B"),
            CBInstruction::RL_C => write!(f, "RL C"),
            CBInstruction::RL_D => write!(f, "RL D"),
            CBInstruction::RL_E => write!(f, "RL E"),
            CBInstruction::RL_H => write!(f, "RL H"),
            CBInstruction::RL_L => write!(f, "RL L"),
            CBInstruction::RL_pHL => write!(f, "RL (pHL)"),
            CBInstruction::RL_A => write!(f, "RL A"),
            CBInstruction::RR_B => write!(f, "RR B"),
            CBInstruction::RR_C => write!(f, "RR C"),
            CBInstruction::RR_D => write!(f, "RR D"),
            CBInstruction::RR_E => write!(f, "RR E"),
            CBInstruction::RR_H => write!(f, "RR H"),
            CBInstruction::RR_L => write!(f, "RR L"),
            CBInstruction::RR_pHL => write!(f, "RR (pHL)"),
            CBInstruction::RR_A => write!(f, "RR A"),
            CBInstruction::SLA_B => write!(f, "SLA B"),
            CBInstruction::SLA_C => write!(f, "SLA C"),
            CBInstruction::SLA_D => write!(f, "SLA D"),
            CBInstruction::SLA_E => write!(f, "SLA E"),
            CBInstruction::SLA_H => write!(f, "SLA H"),
            CBInstruction::SLA_L => write!(f, "SLA L"),
            CBInstruction::SLA_pHL => write!(f, "SLA (pHL)"),
            CBInstruction::SLA_A => write!(f, "SLA A"),
            CBInstruction::SRA_B => write!(f, "SRA B"),
            CBInstruction::SRA_C => write!(f, "SRA C"),
            CBInstruction::SRA_D => write!(f, "SRA D"),
            CBInstruction::SRA_E => write!(f, "SRA E"),
            CBInstruction::SRA_H => write!(f, "SRA H"),
            CBInstruction::SRA_L => write!(f, "SRA L"),
            CBInstruction::SRA_pHL => write!(f, "SRA (pHL)"),
            CBInstruction::SRA_A => write!(f, "SRA A"),
            CBInstruction::SWAP_B => write!(f, "SWAP B"),
            CBInstruction::SWAP_C => write!(f, "SWAP C"),
            CBInstruction::SWAP_D => write!(f, "SWAP D"),
            CBInstruction::SWAP_E => write!(f, "SWAP E"),
            CBInstruction::SWAP_H => write!(f, "SWAP H"),
            CBInstruction::SWAP_L => write!(f, "SWAP L"),
            CBInstruction::SWAP_pHL => write!(f, "SWAP (pHL)"),
            CBInstruction::SWAP_A => write!(f, "SWAP A"),
            CBInstruction::SRL_B => write!(f, "SRL B"),
            CBInstruction::SRL_C => write!(f, "SRL C"),
            CBInstruction::SRL_D => write!(f, "SRL D"),
            CBInstruction::SRL_E => write!(f, "SRL E"),
            CBInstruction::SRL_H => write!(f, "SRL H"),
            CBInstruction::SRL_L => write!(f, "SRL L"),
            CBInstruction::SRL_pHL => write!(f, "SRL (pHL)"),
            CBInstruction::SRL_A => write!(f, "SRL A"),
            CBInstruction::BIT_0_B => write!(f, "BIT 0,B"),
            CBInstruction::BIT_0_C => write!(f, "BIT 0,C"),
            CBInstruction::BIT_0_D => write!(f, "BIT 0,D"),
            CBInstruction::BIT_0_E => write!(f, "BIT 0,E"),
            CBInstruction::BIT_0_H => write!(f, "BIT 0,H"),
            CBInstruction::BIT_0_L => write!(f, "BIT 0,L"),
            CBInstruction::BIT_0_pHL => write!(f, "BIT 0,(pHL)"),
            CBInstruction::BIT_0_A => write!(f, "BIT 0,A"),
            CBInstruction::BIT_1_B => write!(f, "BIT 1,B"),
            CBInstruction::BIT_1_C => write!(f, "BIT 1,C"),
            CBInstruction::BIT_1_D => write!(f, "BIT 1,D"),
            CBInstruction::BIT_1_E => write!(f, "BIT 1,E"),
            CBInstruction::BIT_1_H => write!(f, "BIT 1,H"),
            CBInstruction::BIT_1_L => write!(f, "BIT 1,L"),
            CBInstruction::BIT_1_pHL => write!(f, "BIT 1,(pHL)"),
            CBInstruction::BIT_1_A => write!(f, "BIT 1,A"),
            CBInstruction::BIT_2_B => write!(f, "BIT 2,B"),
            CBInstruction::BIT_2_C => write!(f, "BIT 2,C"),
            CBInstruction::BIT_2_D => write!(f, "BIT 2,D"),
            CBInstruction::BIT_2_E => write!(f, "BIT 2,E"),
            CBInstruction::BIT_2_H => write!(f, "BIT 2,H"),
            CBInstruction::BIT_2_L => write!(f, "BIT 2,L"),
            CBInstruction::BIT_2_pHL => write!(f, "BIT 2,(pHL)"),
            CBInstruction::BIT_2_A => write!(f, "BIT 2,A"),
            CBInstruction::BIT_3_B => write!(f, "BIT 3,B"),
            CBInstruction::BIT_3_C => write!(f, "BIT 3,C"),
            CBInstruction::BIT_3_D => write!(f, "BIT 3,D"),
            CBInstruction::BIT_3_E => write!(f, "BIT 3,E"),
            CBInstruction::BIT_3_H => write!(f, "BIT 3,H"),
            CBInstruction::BIT_3_L => write!(f, "BIT 3,L"),
            CBInstruction::BIT_3_pHL => write!(f, "BIT 3,(pHL)"),
            CBInstruction::BIT_3_A => write!(f, "BIT 3,A"),
            CBInstruction::BIT_4_B => write!(f, "BIT 4,B"),
            CBInstruction::BIT_4_C => write!(f, "BIT 4,C"),
            CBInstruction::BIT_4_D => write!(f, "BIT 4,D"),
            CBInstruction::BIT_4_E => write!(f, "BIT 4,E"),
            CBInstruction::BIT_4_H => write!(f, "BIT 4,H"),
            CBInstruction::BIT_4_L => write!(f, "BIT 4,L"),
            CBInstruction::BIT_4_pHL => write!(f, "BIT 4,(pHL)"),
            CBInstruction::BIT_4_A => write!(f, "BIT 4,A"),
            CBInstruction::BIT_5_B => write!(f, "BIT 5,B"),
            CBInstruction::BIT_5_C => write!(f, "BIT 5,C"),
            CBInstruction::BIT_5_D => write!(f, "BIT 5,D"),
            CBInstruction::BIT_5_E => write!(f, "BIT 5,E"),
            CBInstruction::BIT_5_H => write!(f, "BIT 5,H"),
            CBInstruction::BIT_5_L => write!(f, "BIT 5,L"),
            CBInstruction::BIT_5_pHL => write!(f, "BIT 5,(pHL)"),
            CBInstruction::BIT_5_A => write!(f, "BIT 5,A"),
            CBInstruction::BIT_6_B => write!(f, "BIT 6,B"),
            CBInstruction::BIT_6_C => write!(f, "BIT 6,C"),
            CBInstruction::BIT_6_D => write!(f, "BIT 6,D"),
            CBInstruction::BIT_6_E => write!(f, "BIT 6,E"),
            CBInstruction::BIT_6_H => write!(f, "BIT 6,H"),
            CBInstruction::BIT_6_L => write!(f, "BIT 6,L"),
            CBInstruction::BIT_6_pHL => write!(f, "BIT 6,(pHL)"),
            CBInstruction::BIT_6_A => write!(f, "BIT 6,A"),
            CBInstruction::BIT_7_B => write!(f, "BIT 7,B"),
            CBInstruction::BIT_7_C => write!(f, "BIT 7,C"),
            CBInstruction::BIT_7_D => write!(f, "BIT 7,D"),
            CBInstruction::BIT_7_E => write!(f, "BIT 7,E"),
            CBInstruction::BIT_7_H => write!(f, "BIT 7,H"),
            CBInstruction::BIT_7_L => write!(f, "BIT 7,L"),
            CBInstruction::BIT_7_pHL => write!(f, "BIT 7,(pHL)"),
            CBInstruction::BIT_7_A => write!(f, "BIT 7,A"),
            CBInstruction::RES_0_B => write!(f, "RES 0,B"),
            CBInstruction::RES_0_C => write!(f, "RES 0,C"),
            CBInstruction::RES_0_D => write!(f, "RES 0,D"),
            CBInstruction::RES_0_E => write!(f, "RES 0,E"),
            CBInstruction::RES_0_H => write!(f, "RES 0,H"),
            CBInstruction::RES_0_L => write!(f, "RES 0,L"),
            CBInstruction::RES_0_pHL => write!(f, "RES 0,(pHL)"),
            CBInstruction::RES_0_A => write!(f, "RES 0,A"),
            CBInstruction::RES_1_B => write!(f, "RES 1,B"),
            CBInstruction::RES_1_C => write!(f, "RES 1,C"),
            CBInstruction::RES_1_D => write!(f, "RES 1,D"),
            CBInstruction::RES_1_E => write!(f, "RES 1,E"),
            CBInstruction::RES_1_H => write!(f, "RES 1,H"),
            CBInstruction::RES_1_L => write!(f, "RES 1,L"),
            CBInstruction::RES_1_pHL => write!(f, "RES 1,(pHL)"),
            CBInstruction::RES_1_A => write!(f, "RES 1,A"),
            CBInstruction::RES_2_B => write!(f, "RES 2,B"),
            CBInstruction::RES_2_C => write!(f, "RES 2,C"),
            CBInstruction::RES_2_D => write!(f, "RES 2,D"),
            CBInstruction::RES_2_E => write!(f, "RES 2,E"),
            CBInstruction::RES_2_H => write!(f, "RES 2,H"),
            CBInstruction::RES_2_L => write!(f, "RES 2,L"),
            CBInstruction::RES_2_pHL => write!(f, "RES 2,(pHL)"),
            CBInstruction::RES_2_A => write!(f, "RES 2,A"),
            CBInstruction::RES_3_B => write!(f, "RES 3,B"),
            CBInstruction::RES_3_C => write!(f, "RES 3,C"),
            CBInstruction::RES_3_D => write!(f, "RES 3,D"),
            CBInstruction::RES_3_E => write!(f, "RES 3,E"),
            CBInstruction::RES_3_H => write!(f, "RES 3,H"),
            CBInstruction::RES_3_L => write!(f, "RES 3,L"),
            CBInstruction::RES_3_pHL => write!(f, "RES 3,(pHL)"),
            CBInstruction::RES_3_A => write!(f, "RES 3,A"),
            CBInstruction::RES_4_B => write!(f, "RES 4,B"),
            CBInstruction::RES_4_C => write!(f, "RES 4,C"),
            CBInstruction::RES_4_D => write!(f, "RES 4,D"),
            CBInstruction::RES_4_E => write!(f, "RES 4,E"),
            CBInstruction::RES_4_H => write!(f, "RES 4,H"),
            CBInstruction::RES_4_L => write!(f, "RES 4,L"),
            CBInstruction::RES_4_pHL => write!(f, "RES 4,(pHL)"),
            CBInstruction::RES_4_A => write!(f, "RES 4,A"),
            CBInstruction::RES_5_B => write!(f, "RES 5,B"),
            CBInstruction::RES_5_C => write!(f, "RES 5,C"),
            CBInstruction::RES_5_D => write!(f, "RES 5,D"),
            CBInstruction::RES_5_E => write!(f, "RES 5,E"),
            CBInstruction::RES_5_H => write!(f, "RES 5,H"),
            CBInstruction::RES_5_L => write!(f, "RES 5,L"),
            CBInstruction::RES_5_pHL => write!(f, "RES 5,(pHL)"),
            CBInstruction::RES_5_A => write!(f, "RES 5,A"),
            CBInstruction::RES_6_B => write!(f, "RES 6,B"),
            CBInstruction::RES_6_C => write!(f, "RES 6,C"),
            CBInstruction::RES_6_D => write!(f, "RES 6,D"),
            CBInstruction::RES_6_E => write!(f, "RES 6,E"),
            CBInstruction::RES_6_H => write!(f, "RES 6,H"),
            CBInstruction::RES_6_L => write!(f, "RES 6,L"),
            CBInstruction::RES_6_pHL => write!(f, "RES 6,(pHL)"),
            CBInstruction::RES_6_A => write!(f, "RES 6,A"),
            CBInstruction::RES_7_B => write!(f, "RES 7,B"),
            CBInstruction::RES_7_C => write!(f, "RES 7,C"),
            CBInstruction::RES_7_D => write!(f, "RES 7,D"),
            CBInstruction::RES_7_E => write!(f, "RES 7,E"),
            CBInstruction::RES_7_H => write!(f, "RES 7,H"),
            CBInstruction::RES_7_L => write!(f, "RES 7,L"),
            CBInstruction::RES_7_pHL => write!(f, "RES 7,(pHL)"),
            CBInstruction::RES_7_A => write!(f, "RES 7,A"),
            CBInstruction::SET_0_B => write!(f, "SET 0,B"),
            CBInstruction::SET_0_C => write!(f, "SET 0,C"),
            CBInstruction::SET_0_D => write!(f, "SET 0,D"),
            CBInstruction::SET_0_E => write!(f, "SET 0,E"),
            CBInstruction::SET_0_H => write!(f, "SET 0,H"),
            CBInstruction::SET_0_L => write!(f, "SET 0,L"),
            CBInstruction::SET_0_pHL => write!(f, "SET 0,(pHL)"),
            CBInstruction::SET_0_A => write!(f, "SET 0,A"),
            CBInstruction::SET_1_B => write!(f, "SET 1,B"),
            CBInstruction::SET_1_C => write!(f, "SET 1,C"),
            CBInstruction::SET_1_D => write!(f, "SET 1,D"),
            CBInstruction::SET_1_E => write!(f, "SET 1,E"),
            CBInstruction::SET_1_H => write!(f, "SET 1,H"),
            CBInstruction::SET_1_L => write!(f, "SET 1,L"),
            CBInstruction::SET_1_pHL => write!(f, "SET 1,(pHL)"),
            CBInstruction::SET_1_A => write!(f, "SET 1,A"),
            CBInstruction::SET_2_B => write!(f, "SET 2,B"),
            CBInstruction::SET_2_C => write!(f, "SET 2,C"),
            CBInstruction::SET_2_D => write!(f, "SET 2,D"),
            CBInstruction::SET_2_E => write!(f, "SET 2,E"),
            CBInstruction::SET_2_H => write!(f, "SET 2,H"),
            CBInstruction::SET_2_L => write!(f, "SET 2,L"),
            CBInstruction::SET_2_pHL => write!(f, "SET 2,(pHL)"),
            CBInstruction::SET_2_A => write!(f, "SET 2,A"),
            CBInstruction::SET_3_B => write!(f, "SET 3,B"),
            CBInstruction::SET_3_C => write!(f, "SET 3,C"),
            CBInstruction::SET_3_D => write!(f, "SET 3,D"),
            CBInstruction::SET_3_E => write!(f, "SET 3,E"),
            CBInstruction::SET_3_H => write!(f, "SET 3,H"),
            CBInstruction::SET_3_L => write!(f, "SET 3,L"),
            CBInstruction::SET_3_pHL => write!(f, "SET 3,(pHL)"),
            CBInstruction::SET_3_A => write!(f, "SET 3,A"),
            CBInstruction::SET_4_B => write!(f, "SET 4,B"),
            CBInstruction::SET_4_C => write!(f, "SET 4,C"),
            CBInstruction::SET_4_D => write!(f, "SET 4,D"),
            CBInstruction::SET_4_E => write!(f, "SET 4,E"),
            CBInstruction::SET_4_H => write!(f, "SET 4,H"),
            CBInstruction::SET_4_L => write!(f, "SET 4,L"),
            CBInstruction::SET_4_pHL => write!(f, "SET 4,(pHL)"),
            CBInstruction::SET_4_A => write!(f, "SET 4,A"),
            CBInstruction::SET_5_B => write!(f, "SET 5,B"),
            CBInstruction::SET_5_C => write!(f, "SET 5,C"),
            CBInstruction::SET_5_D => write!(f, "SET 5,D"),
            CBInstruction::SET_5_E => write!(f, "SET 5,E"),
            CBInstruction::SET_5_H => write!(f, "SET 5,H"),
            CBInstruction::SET_5_L => write!(f, "SET 5,L"),
            CBInstruction::SET_5_pHL => write!(f, "SET 5,(pHL)"),
            CBInstruction::SET_5_A => write!(f, "SET 5,A"),
            CBInstruction::SET_6_B => write!(f, "SET 6,B"),
            CBInstruction::SET_6_C => write!(f, "SET 6,C"),
            CBInstruction::SET_6_D => write!(f, "SET 6,D"),
            CBInstruction::SET_6_E => write!(f, "SET 6,E"),
            CBInstruction::SET_6_H => write!(f, "SET 6,H"),
            CBInstruction::SET_6_L => write!(f, "SET 6,L"),
            CBInstruction::SET_6_pHL => write!(f, "SET 6,(pHL)"),
            CBInstruction::SET_6_A => write!(f, "SET 6,A"),
            CBInstruction::SET_7_B => write!(f, "SET 7,B"),
            CBInstruction::SET_7_C => write!(f, "SET 7,C"),
            CBInstruction::SET_7_D => write!(f, "SET 7,D"),
            CBInstruction::SET_7_E => write!(f, "SET 7,E"),
            CBInstruction::SET_7_H => write!(f, "SET 7,H"),
            CBInstruction::SET_7_L => write!(f, "SET 7,L"),
            CBInstruction::SET_7_pHL => write!(f, "SET 7,(pHL)"),
            CBInstruction::SET_7_A => write!(f, "SET 7,A")
        }
    }
}
