//! This thing generates random voter groups of a given size
//! Given the set of all authorized voters (their pubkeys) it selects one randomly
//! then it picks a shift distance (some prime number less than the voter set size)
//! and iteratively selects the rest of the group by shifting that distance
//! its treating the set of voters as a ring 

use crate::pubkey::{Pubkey};

use std::collections::HashMap;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
//use std::sync::Arc;

pub const OPTIMAL_VOTE_GROUP_SIZE : usize = 11;

//#[derive(Clone, Debug, Serialize, Deserialize, AbiExample, PartialEq)]
//pub struct ArcPubkey(std::sync::Arc<Pubkey>);

#[derive(Clone, Debug, Serialize, Deserialize, AbiExample, PartialEq)]
pub struct VoteGroupGenerator
{
    all_voters: Vec<Pubkey>,
    all_distance: Vec<u32>,
    group_size: usize,
}

impl VoteGroupGenerator
{
    pub fn new (map: &HashMap<Pubkey, Pubkey>, size: usize) -> VoteGroupGenerator {

        let temp1: Vec<_> = map.into_iter().collect();
       // let (temp, _): (Vec<Pubkey>, Vec<Pubkey>) = temp1.iter().cloned().unzip();
       let mut temp = Vec::new();
       for x in temp1{
           let pubk = x.0;
           let cloned :Pubkey= Pubkey::new_from_array(pubk.to_bytes());
           temp.push(cloned);
       }
        let len = temp.len() as u32;
        let mut initial = Vec::new();
        initial.push(1);
        for val in [2,3,5,7,11,13,17,23,29,31,37,41,43,47,51,53,57,59,61,67,71,73,79,83,87,89,97,101,103].iter() {
           if (len > *val) && ((len % *val) != 0)   {
               initial.push(*val);           
           }
        }
        Self { all_voters : temp,
            all_distance: initial.to_owned(),
            group_size: size
        } 
    }

    pub fn new_dummy () -> VoteGroupGenerator {
        let hm: HashMap<Pubkey,Pubkey> = HashMap::new();
        Self::new(&hm,1)
    }

    fn ring_shift(&self,a : usize,b: usize) -> usize{
       let temp = a + b;
        temp % self.all_voters.len()
    }
/* 
    pub fn group_for_seed(&self,seed : u64) -> Vec<Pubkey>{
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(seed); // seed the hash with the slot

         let curhash = hasher.finish();
         let mut ret = Vec::with_capacity(self.group_size);
         let voters_len = self.all_voters.len();
         let mut loc = (curhash % voters_len as u64) as usize;
         ret.push(Pubkey::new(&self.all_voters[loc].to_bytes()));
         if self.group_size > 1 {
            let choose_dist = curhash % self.all_distance.len() as u64;
            let dist = self.all_distance[choose_dist as usize] as usize;
            for _ in 0..(self.group_size -1){
                loc = self.ring_shift(loc,dist);
                let temp = Pubkey::new(&self.all_voters[loc].to_bytes());
                ret.push(temp);
            }
        }
        println!("ret : {:?}", ret);
         ret
    }
*/
    pub fn in_group_for_seed(&self,seed : u64, test_key: Pubkey) -> bool {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(seed); // seed the hash with the slot

         let curhash = hasher.finish();
         let voters_len = self.all_voters.len();
         let mut loc = (curhash % voters_len as u64) as usize;
         let temp = Pubkey::new(&self.all_voters[loc].to_bytes());
         if test_key == temp{
            return true;
        }
         if self.group_size > 1 {
            let choose_dist = curhash % self.all_distance.len() as u64;
            let dist = self.all_distance[choose_dist as usize] as usize;
            for _ in 0..(self.group_size -1){
                loc = self.ring_shift(loc,dist);
                let temp = Pubkey::new(&self.all_voters[loc].to_bytes());
                if test_key == temp{
                    println!("found {:?}",test_key);
                    return true;
                }
            }
        }
        false

    }
}
pub mod tests {
    use super::*;

    #[test]
    fn test_vgg_multi() {
        let canary = Pubkey::new_unique();
        let mut hm: HashMap<Pubkey,Pubkey> = HashMap::new();
        hm.insert(canary, Pubkey::new_unique());

        for it in 0..4{
            let val = Pubkey::new_unique();
            hm.insert(val, Pubkey::new_unique());
            println!("insert {}",it);
        }
        let vgg = VoteGroupGenerator::new(&hm,hm.len());
        for h in hm.keys() {
            let found = vgg.in_group_for_seed(0, *h);
            assert!(found);
        }

        let not_canary = Pubkey::new_unique();
        assert_eq!(vgg.in_group_for_seed(0, not_canary),false);
    }

    #[test]

    fn test_vgg_single() {
        let canary = Pubkey::new_unique();
        let mut hm: HashMap<Pubkey,Pubkey> = HashMap::new();
        hm.insert(canary, Pubkey::new_unique());

        let vgg = VoteGroupGenerator::new(&hm,hm.len());
        for h in hm.keys() {
            let found = vgg.in_group_for_seed(0, *h);
            assert!(found);
        }

        let not_canary = Pubkey::new_unique();
        assert_eq!(vgg.in_group_for_seed(0, not_canary),false);
    }

}
