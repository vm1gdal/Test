// #![allow(dead_code)]
#![allow(non_snake_case)]

use super::digest::*;
use super::transaction::*;
use std::borrow::{ Borrow, BorrowMut };

#[ repr( C ) ]
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BlockGeneric< Body >
{
  pub time : i64,
  pub nonce : u32,
  pub pre_hash : Digest,
  pub merkle_hash : Digest,
  pub difficulty : u32,
  pub body : Body,
}

pub type BlockHeader = BlockGeneric< () >;

#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BlockBody
{
  pub transactions : Vec< Transaction >,
  pub hash : Digest,
}

pub type Block = BlockGeneric< BlockBody >;

//

impl BlockHeader
{
  pub fn proof_of_work( &mut self ) -> Digest
  {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/2
    complexity : difficult
    stage : early
    */
    Digest::new()
  }
  pub fn form( self ) -> Block
  {
    Block::new( self )
  }
}

//

impl Block
{

  pub fn new( header : BlockHeader ) -> Self
  {
    let body = BlockBody
    {
      transactions : vec![],
      hash : Digest::new(),
    };
    Self
    {
      time : header.time,
      nonce : header.nonce,
      pre_hash : header.pre_hash,
      merkle_hash : header.merkle_hash,
      difficulty : header.difficulty,
      body,
    }
  }

  pub fn header( &mut self ) -> &mut BlockHeader
  {
    self.borrow_mut()
  }

}

//

impl Borrow< BlockHeader > for Block
{
  fn borrow<'a>( &'a self ) -> &'a BlockHeader
  {
    // we want zero-copy. instead of making a copy we reinterpret the larget structure containg smaller
    unsafe
    {
      std::mem::transmute::< &'a Block, &'a BlockHeader >( self )
    }
  }
}

//

impl BorrowMut< BlockHeader > for Block
{
  fn borrow_mut<'a>( &'a mut self ) -> &'a mut BlockHeader
  {
    // we want zero-copy. instead of making a copy we reinterpret the larget structure containg smaller
    unsafe
    {
      std::mem::transmute::< &'a mut Block, &'a mut BlockHeader >( self )
    }
  }
}
