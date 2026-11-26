//! Read-only legacy u-blox acquisition diagnostics. No receiver settings change.
use raylar_drivers::gps::ublox;

pub const CONFIG_POLLS: &[&[u8]] = &[
    &ublox::packet::<8>(6, 0x24, &[]),  // NAV5
    &ublox::packet::<8>(6, 0x23, &[]),  // NAVX5
    &ublox::packet::<8>(6, 0x3e, &[]),  // GNSS
    &ublox::packet::<8>(6, 8, &[]),     // RATE
    &ublox::packet::<9>(6, 0x31, &[0]), // TP5, timepulse 0
    &ublox::packet::<8>(6, 0x16, &[]),  // SBAS
    &ublox::packet::<8>(6, 0x11, &[]),  // RXM
    &ublox::packet::<8>(6, 0x3b, &[]),  // PM2
    &ublox::packet::<8>(6, 0x39, &[]),  // ITFM
];
pub const STATUS: [u8; 8] = ublox::packet(1, 3, &[]);
pub const SATELLITES: [u8; 8] = ublox::packet(1, 0x30, &[]);
pub const HARDWARE: [u8; 8] = ublox::packet(10, 9, &[]);
pub const DOP: [u8; 8] = ublox::packet(1, 4, &[]);
pub const GPS_TIME: [u8; 8] = ublox::packet(1, 0x20, &[]);
pub const UTC_TIME: [u8; 8] = ublox::packet(1, 0x21, &[]);
pub const AIDING: [u8; 8] = ublox::packet(11, 2, &[]);

fn u16_at(p: &[u8], i: usize) -> u16 {
    u16::from_le_bytes(p[i..i + 2].try_into().unwrap())
}
fn u32_at(p: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(p[i..i + 4].try_into().unwrap())
}

/// Called only for checksum-validated frames. Every decoder checks its bounds.
pub fn frame(class: u8, id: u8, p: &[u8]) {
    match (class, id) {
        (1, 3) if p.len() == 16 => {
            defmt::info!("GNSS DIAG STATUS tow_ms={} fix={} flags={:02x} fix_stat={:02x} flags2={:02x} ttff_ms={} uptime_ms={}", u32_at(p,0), p[4],p[5],p[6],p[7],u32_at(p,8),u32_at(p,12));
        }
        (1, 4) if p.len() == 18 => {
            defmt::info!("GNSS DIAG DOP tow_ms={} gdop_centi={} pdop_centi={} tdop_centi={} vdop_centi={} hdop_centi={}",u32_at(p,0),u16_at(p,4),u16_at(p,6),u16_at(p,8),u16_at(p,10),u16_at(p,12));
        }
        (1, 0x20) if p.len() == 16 => {
            defmt::info!(
                "GNSS DIAG GPS_TIME tow_ms={} week={} leap_seconds={} valid={:02x} accuracy_ns={}",
                u32_at(p, 0),
                u16_at(p, 8) as i16,
                p[10] as i8,
                p[11],
                u32_at(p, 12)
            );
        }
        (1, 0x21) if p.len() == 20 => {
            defmt::info!("GNSS DIAG UTC_TIME tow_ms={} accuracy_ns={} year={} month={} day={} hour={} minute={} second={} valid={:02x}",u32_at(p,0),u32_at(p,4),u16_at(p,12),p[14],p[15],p[16],p[17],p[18],p[19]);
        }
        (11, 2) if p.len() == 72 => {
            defmt::info!(
                "GNSS DIAG AID_HUI health={:08x} utc_week={} utc_leap_seconds={} flags={:08x}",
                u32_at(p, 0),
                u16_at(p, 24) as i16,
                u16_at(p, 26) as i16,
                u32_at(p, 68)
            );
        }
        (1, 0x30) if p.len() >= 8 && p.len() == 8 + usize::from(p[4]) * 12 => {
            let mut signals = 0u8;
            let mut used = 0u8;
            let mut code_locked = 0u8;
            let mut ephemeris = 0u8;
            let mut strongest = 0u8;
            for s in p[8..].chunks_exact(12) {
                signals += u8::from(s[4] > 0);
                used += s[2] & 1;
                code_locked += u8::from(s[3] & 7 >= 4);
                ephemeris += u8::from(s[2] & 12 == 12);
                strongest = strongest.max(s[4]);
            }
            defmt::info!("GNSS DIAG SVINFO tow_ms={} channels={} signals={} used={} code_locked={} ephemeris={} strongest_cno={} global_flags={:02x}",u32_at(p,0),p[4],signals,used,code_locked,ephemeris,strongest,p[5]);
            for s in p[8..].chunks_exact(12) {
                if s[1] == 0 || s[1] == 255 {
                    continue;
                }
                defmt::info!("GNSS DIAG SV id={} channel={} cno={} quality={} used={} flags={:02x} elev={} az={} residual_cm={}",s[1],s[0],s[4],s[3]&7,s[2]&1!=0,s[2],s[5] as i8,u16_at(s,6) as i16,u32_at(s,8) as i32);
            }
        }
        (10, 9) if p.len() == 60 || p.len() == 68 => {
            let jam = if p.len() == 60 { p[45] } else { p[53] };
            defmt::info!("GNSS DIAG HW noise={} agc={} antenna_status={} antenna_power={} flags={:02x} jam_ind={}",u16_at(p,16),u16_at(p,18),p[20],p[21],p[22],jam);
        }
        (6, 0x24) if p.len() == 36 => {
            defmt::info!("GNSS DIAG NAV5 dyn_model={} fix_mode={} min_elev={} pdop_mask_deci={} tdop_mask_deci={} position_accuracy_m={} time_accuracy_m={} cno_threshold_svs={} cno_threshold={}",p[2],p[3],p[12] as i8,u16_at(p,14),u16_at(p,16),u16_at(p,18),u16_at(p,20),p[24],p[25]);
        }
        (6, 0x23) if p.len() == 40 => {
            defmt::info!("GNSS DIAG NAVX5 version={} min_svs={} max_svs={} min_cno={} initial_fix_3d={} word18={} ppp={} aop={}",u16_at(p,0),p[10],p[11],p[12],p[14],u16_at(p,18),p[26],p[27]);
        }
        (6, 0x3e) if p.len() >= 4 && p.len() == 4 + usize::from(p[3]) * 8 => {
            defmt::info!(
                "GNSS DIAG CONFIG_GNSS version={} hardware_channels={} used_channels={} blocks={}",
                p[0],
                p[1],
                p[2],
                p[3]
            );
            for g in p[4..].chunks_exact(8) {
                defmt::info!("GNSS DIAG CONSTELLATION id={} enabled={} reserved_channels={} max_channels={} flags={:08x}",g[0],g[4]&1!=0,g[1],g[2],u32_at(g,4));
            }
        }
        (6, 8) if p.len() == 6 => {
            defmt::info!(
                "GNSS DIAG RATE measurement_ms={} navigation_cycles={} time_reference={}",
                u16_at(p, 0),
                u16_at(p, 2),
                u16_at(p, 4)
            );
        }
        (6, 0x31) if p.len() == 32 => {
            defmt::info!("GNSS DIAG TP5 index={} version={} unlocked_freq_period={} locked_freq_period={} unlocked_length_ratio={} locked_length_ratio={} user_delay_ns={} flags={:08x}",p[0],p[1],u32_at(p,8),u32_at(p,12),u32_at(p,16),u32_at(p,20),u32_at(p,24) as i32,u32_at(p,28));
        }
        (6, 0x16) if p.len() == 8 => {
            defmt::info!(
                "GNSS DIAG SBAS mode={:02x} usage={:02x} max_sbas={} scan2={:02x} scan1={:08x}",
                p[0],
                p[1],
                p[2],
                p[3],
                u32_at(p, 4)
            );
        }
        (6, 0x11) if p.len() == 2 => {
            defmt::info!("GNSS DIAG RXM low_power_mode={}", p[1]);
        }
        (6, 0x3b) if p.len() >= 24 => {
            defmt::info!("GNSS DIAG PM2 version={} flags={:08x} update_ms={} search_ms={} grid_ms={} on_s={} min_acquisition_s={}",p[0],u32_at(p,4),u32_at(p,8),u32_at(p,12),u32_at(p,16),u16_at(p,20),u16_at(p,22));
        }
        (6, 0x39) if p.len() == 8 => {
            defmt::info!(
                "GNSS DIAG ITFM config={:08x} config2={:08x}",
                u32_at(p, 0),
                u32_at(p, 4)
            );
        }
        (5, 0) if p.len() == 2 => {
            defmt::info!("GNSS DIAG NAK class={} id={}", p[0], p[1]);
        }
        _ => {}
    }
}
