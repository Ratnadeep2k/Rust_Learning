fn main() {
    // let mut x:i32 =4; //i:32  is 32 bit integer 
    // println!("Hello, world! is {}",x);
    // x =3;
    // println!("Reassign Value using mut {}",x);
     //Tuples 

     let a = (5,"Hello world",false);
     println!("The first value is {}",a.0);
     println!("The second value is {}",a.1);
     println!("The third value is {}",a.2);
 

     let b = (1,2,3);

     //option 1
     let val_one= b.0;
     let val_two= b.1;
     println!("The first value is {},and second value is {}",val_one,val_two);

     //destructre 
     let c =(5,6,7);
     let(val_one,val_two,_) = c; //ignore the last value ;
        println!("The first value is {},and second value is {}",val_one,val_two);

        //Array 

     let  d = [1,2,3,4,5];
        println!("The first value is {}",d[0]);


}
