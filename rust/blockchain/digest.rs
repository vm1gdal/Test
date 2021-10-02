#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use sha2::{ Sha256, Digest as ShaDigest };
use std::fmt::{ self, Write };
use std::ops::Deref;
use std::borrow::Borrow;

use super::transaction::*;

//

#[ derive( Clone, Serialize, Deserialize, Eq, PartialEq, Hash ) ]
pub struct Digest ( Vec< u8 > );

impl From< Vec< u8 > > for Digest
{
  fn from( src : Vec< u8 > ) -> Self
  {
    Digest ( src )
  }
}

//

impl Deref for Digest
{
  type Target = Vec< u8 >;

  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }

}

//

impl Borrow< [ u8 ] > for Digest
{
  fn borrow( &self ) -> &[ u8 ]
  {
    &self.0
  }
}

//

impl fmt::Debug for Digest
{
  fn fmt( &self, f : &mut fmt::Formatter<'_>) -> fmt::Result
  {
    f.write_str( &bytes_to_string_hex( &self.0 ) )
  }
}

//

impl fmt::Display for Digest
{
  fn fmt( &self, f : &mut fmt::Formatter<'_>) -> fmt::Result
  {
    f.write_str( &bytes_to_string_hex( &self.0 ) )
  }
}

//

impl Digest
{

  pub fn new() -> Self
  {
    Self ( Vec::new() )
  }

}

//

pub fn hash_single< T : serde::Serialize >( _item : &T ) -> Digest
{
  /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/5
    complexity : medium
    stage : early
  */
  Digest::new()
}

//

pub fn hash_every< T : serde::Serialize + fmt::Debug >( _items : &Vec< T > ) -> Digest
{
  /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/8
    complexity : medium
    stage : early
  */

  Digest::new()

}

//

pub fn bytes_to_string_hex( _src : &[ u8 ] ) -> String
{
  /*
  issue : https://github.com/Learn-Together-Pro/Blockchain/issues/10
  complexity : difficult
  stage : early
  */
  String::new()
}

//

pub fn merkle_calc( _transactions : &Vec< Transaction > ) -> Digest
{
  let zero : Vec< u8 > = [ 0 ; 64 ].into();
  zero.into()
  /*
  issue : https://github.com/Learn-Together-Pro/Blockchain/issues/6
  complexity : mid
  stage : early
  */
}
