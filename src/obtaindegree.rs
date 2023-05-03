use crate::readfile::Graph;

//Step3: Determine the degrees of each node
pub fn getdegrees(graph:Graph)->Vec<usize>{
    let mut listofdegrees=vec![];
    for i in 0..4039{
        listofdegrees.push(graph.outedges[i].len());
    }
    listofdegrees 
    //Each number in this list correspond to the degrees of the corresponding node.
    //The 0th number of the list will show the degree of the 0th node, and so on
}

//Step 4: Obtain occurance for each degree
pub fn count_occurance(initialvec:&Vec<usize>)->Vec<(u32,u32)>{
    let mut occurancecount:Vec<(u32,u32)>=vec![];
    for i in 0..initialvec.len(){
        let mut found=false;
        for j in 0..occurancecount.len(){
            if occurancecount[j].0==initialvec[i] as u32{
                occurancecount[j].1=occurancecount[j].1+1;
                found=true;
                break
            }
        }
        if !found{
            occurancecount.push((initialvec[i] as u32,1));
        }
    }
    occurancecount.sort_by_key(|k| k.0);
    occurancecount
    //The final output of this function is to return a vector that shows the count for x number of connected nodes
    //Thus, the first number of the tuple is the number of friends(ex: 1 friend, 2 friends...)
    //The second number of the tuple is the number of occurance(ex: how many people have 1 friend and so on)
}

#[test]
fn testoccurance(){
    let testvec:Vec<usize>=vec![1,1,1,2,2,3,3,4,5];
    //the count for each number (in order) should be 3,2,2,1,1
    let testresult=count_occurance(&testvec);
    assert_eq!(testresult[0].1,3 as u32,"The occurance function is wrong.")
}

//Obtain information above, but the output is in order of highest occurance number to lowest.
pub fn occurance_order(countvec:&Vec<(u32,u32)>)->Vec<(u32,u32)>{
    let mut orderlist=countvec.clone();
    orderlist.sort_by_key(|k| k.1);
    orderlist.reverse();
    return orderlist
}
