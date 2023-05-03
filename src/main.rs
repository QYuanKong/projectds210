mod obtaindegree;
mod readfile;
mod calculation;

//Project Research Question: Determine whether the degree of nodes of the facebook friends graph follows the power distribution.

fn main(){
    use readfile::makeadjancencylist;
    use obtaindegree::getdegrees;
    use obtaindegree::count_occurance;
    use obtaindegree::occurance_order;
    use calculation::seperatexy;
    use calculation::logthexy;
    use calculation::rsquare;


    let mygraph=makeadjancencylist(); 
    //Make adjacency list for graph 

    let myvec=getdegrees(mygraph); 
    //get the degree of vertexes for graph

    let counts = count_occurance(&myvec); 
    //get the count of occurances of each degree
    let ordercount=occurance_order(&counts);
    println!("The top 1 occurance of degree of nodes is {}, with occurance of {} times in this dataset.",ordercount[0].0,ordercount[0].1);
    println!("The top 2 occurance of degree of nodes is {}, with occurance of {} times in this dataset.",ordercount[1].0,ordercount[1].1);
    println!("The top 3 occurance of degree of nodes is {}, with occurance of {} times in this dataset.",ordercount[2].0,ordercount[2].1);
    
    let (myx, myy)=seperatexy(counts);
    //seperate the x and y to test so could be later used to calculate r square value

    let (mylnx,mylny)=logthexy(&myx,&myy);
    //Since we assume this data follows a power distribution
    //the x and y are transformed into lnx and lny to reconstruct as linear

    let myrsquare=rsquare(&mylnx,&mylny);
    //And the r square value for lnx and lny is calculated
    //The larger the r square value, indicates how well the data fits the model. 
    println!("The R square value of this data set is {}.This suggests roughly {}% of the data fits the power distribution model.",myrsquare, myrsquare*100.0)
    
}
