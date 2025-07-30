fn main() {
    let a=1000;
    println!("Scope in out={}",a);
    {
      let a=1100;
      println!("Scope in out={}",a);
    }
    println!("Scope in out={}",a);
    
}
//Created by RajeshLingala