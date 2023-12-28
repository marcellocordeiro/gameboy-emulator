use self::structs::parse_tests;
use crate::cpu::{alu::*, registers::Flags};

mod deserializers;
mod structs;

#[test]
fn test_add() {
    let tests = parse_tests("add");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = add(&mut flags, test.x, test.y);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_adc() {
    let tests = parse_tests("adc");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = adc(&mut flags, test.x, test.y);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_sub() {
    let tests = parse_tests("sub");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = sub(&mut flags, test.x, test.y);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_sbc() {
    let tests = parse_tests("sbc");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = sbc(&mut flags, test.x, test.y);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_cp() {
    let tests = parse_tests("cp");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        cp(&mut flags, test.x, test.y);

        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_and() {
    let tests = parse_tests("and");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = and(&mut flags, test.x, test.y);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_or() {
    let tests = parse_tests("or");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = or(&mut flags, test.x, test.y);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_xor() {
    let tests = parse_tests("xor");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = xor(&mut flags, test.x, test.y);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_ccf() {
    let tests = parse_tests("ccf");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        ccf(&mut flags);

        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_scf() {
    let tests = parse_tests("scf");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        scf(&mut flags);

        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_cpl() {
    let tests = parse_tests("cpl");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = cpl(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_daa() {
    let tests = parse_tests("daa");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = daa(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_rl() {
    let tests = parse_tests("rl");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = rl(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_rr() {
    let tests = parse_tests("rr");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = rr(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_rlc() {
    let tests = parse_tests("rlc");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = rlc(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_rrc() {
    let tests = parse_tests("rrc");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = rrc(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_srl() {
    let tests = parse_tests("srl");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = srl(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_sra() {
    let tests = parse_tests("sra");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = sra(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_sla() {
    let tests = parse_tests("sla");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = sla(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_bit() {
    let tests = parse_tests("bit");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        bit(&mut flags, test.y as usize, test.x);

        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_res() {
    let tests = parse_tests("res");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = res(&mut flags, test.y as usize, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_set() {
    let tests = parse_tests("set");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = set(&mut flags, test.y as usize, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}

#[test]
fn test_swap() {
    let tests = parse_tests("swap");

    for (i, test) in tests.into_iter().enumerate() {
        let mut flags = Flags::from_bits_truncate(test.flags);
        let result = swap(&mut flags, test.x);

        assert_eq!(result, test.result.value, "Test #{i} failed.");
        assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
    }
}
