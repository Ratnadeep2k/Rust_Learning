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

        // A set of same value;
     let e  = [9;10]; //9 , 10 times 
     println!("The Array is {:?}",e);

     let num = 1;
     if num ==1 {
      println!("You Won {}",num);
     }
     else {
      println!("You Lost");
     }

     //loop

     //multiply x till it is less than 5000]
     let mut x= 3;
     loop{
      x = x * 2;
      if x > 5000 {
        break;
      }
      println!("The value of x is{}",x);
     }
     //while loop
     let mut y =1;
     while y<1000{
      y = y * 2;
      println!("The value of y is{}",y);
     }

     //for loop 

     for z in 0 ..10{ //from 0 to 9 the right side is exclusive
      println!("The value of z is{}",z);
     }

     // for loop extra use

      for w in 0 ..=9{ //{inclusive}
      println!("The value of w is{}",w);
      }



}
