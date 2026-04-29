fn main() {
   // let x=4;
   /* if{
        println!("TRUE");
    }
    else {
        println!("False");
    }*/
    //if x!=0{
     //   println!("hELLLO");
 //   }
 let mut cnt=0;
 let res=loop{
    cnt+=1;
    if cnt==10{
        break cnt*2;
    }
 };
 println!("the result is : {res}");
 let mut n=4;
 while n !=0{
    println!("{n}");
    n-=1;
 }
 println!("helooo! completed loop");
 let b=[1,2,3 ,4,5];
 for a in b{
    println!("The value of element is:{a}");
 }
 for number in (1..4).rev() {//.rev() reverse the range and 1 to 4 is the range and 4 is excluded;
        println!("{number}! ha ha reversed ");
    }
    println!("LIFTOFF!!!");
}
//
