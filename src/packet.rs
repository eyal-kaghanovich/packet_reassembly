/* Eyal Kaghanovich
 * process fragments and reconstruct packet
*/


// flow:
// build_map(frags: &[Fragment]) -> Result<Map<Fragment>, AssemblyErr>,
// assemble_packet(packet_map: Map<Fragment>)

use std::{collections::BTreeMap, fs::Metadata, mem::offset_of};
use std::collections::btree_map::Entry;


enum AssemblyErr {
    PacketCollisionDetected(usize), // wraps the position of the colliding fragments
    PacketGapDetected(usize), // wraps the offset of the gap
}

struct Fragment {
    offset: usize,
    data: Vec<u8>
}

impl Fragment {
    fn new(offset: usize, data: Vec<u8>) -> Self {
        Self {
            offset: offset,
            data: data,
        }
    }
}


/// takes an array of fragments and returns either a BTreeMap of the packet fragments that is
/// guarenteed to be valid or it returns the error with the position at which the conflict/gap
/// takes place, consider changing &Fragment tp Fragment
fn build_map(frags: &[Fragment]) -> Result<BTreeMap<usize, &Fragment>, AssemblyErr> {
    let mut map: BTreeMap<usize, &Fragment> = BTreeMap::new();
    let offset: usize = 0;

    for frag in frags {
        // try to insert frag, if there is a collision return early with
        // Err(AssemblyErr::PacketCollisionDetected)
        match map.entry(offset) {
            Entry::Occupied(_e) => {
                // collision
                return Err(AssemblyErr::PacketCollisionDetected(frag.offset));
            }
            Entry::Vacant(e) => {
                e.insert(frag);
            }
        }
    }
    // check for gaps
    // if there are none: return the map
    
    
    Ok(map)
}

fn assemble_packet(packet_len: usize, mut packet_map: BTreeMap<usize, &Fragment>) -> Option<Vec<u8>> {
    if packet_len == 0 {
        return None;
    }
    let mut buf: Vec<u8> = vec![0u8; packet_len];
    let mut offset: usize = 0;
    
    while offset < packet_len { 
        let frag = packet_map.get(&offset).unwrap();
        let len = frag.data.len();
        
        // buf[offset..offset+len] = frag.data[0..len]
        buf[offset..offset + len].copy_from_slice(&frag.data);
        offset += frag.offset;
    }
    
    Some(buf)
}
