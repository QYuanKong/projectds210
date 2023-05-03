//Step 5: Obtain the x and y seperatly for future use
pub fn seperatexy(countvec:Vec<(u32,u32)>)->(Vec<f64>,Vec<f64>){
    let n=countvec.len();
    let mut xlist=vec![];
    let mut ylist=vec![];
    for i in 0..n{
        xlist.push(countvec[i].0 as f64);
        ylist.push(countvec[i].1 as f64);
    }
    return(xlist,ylist)
}

//Step 6: Change x and y to log x and log y
//If we assume the original relation is power distribution, lnx and lny will form a linear distribution
pub fn logthexy(xlist:&Vec<f64>, ylist:&Vec<f64>)->(Vec<f64>, Vec<f64>){
    let mut lnxlist=vec![];
    let mut lnylist=vec![];
    let n=xlist.len();
    for i in 0..n{
        lnxlist.push(xlist[i].ln());
        lnylist.push(ylist[i].ln());
    }
    return(lnxlist,lnylist)
}

//Step7: Calaculate r square value
pub fn rsquare(lnx:&Vec<f64>, lny:&Vec<f64>)->f64{
    let n=lnx.len();
    //Calculate the sum for x and y
    let xsum=lnx.iter().sum::<f64>();
    let ysum=lny.iter().sum::<f64>();

    //Calculate sum of xy, sum of xsquare and sum of ysquare
    let mut xyvec:Vec<f64>=vec![];
    for i in 0..n{
        xyvec.push(lnx[i]*lny[i])
    }
    let xysum=xyvec.iter().sum::<f64>();

    let mut xsquare:Vec<f64>=vec![];
    let mut ysquare:Vec<f64>=vec![];
    for i in 0..n{
        xsquare.push((lnx[i]).powi(2));
        ysquare.push((lny[i]).powi(2));
    }
    let xsquaresum=xsquare.iter().sum::<f64>();
    let ysquaresum=ysquare.iter().sum::<f64>();

    let rvalue=((n as f64 *xysum)-(xsum*ysum))/(((n as f64 *xsquaresum - xsum.powi(2)).sqrt())*((n as f64 *ysquaresum-ysum.powi(2)).sqrt()));
    let rsquare=format!("{:.4}", rvalue.powi(2)).parse::<f64>().unwrap();
    //Keeping the value to four decimal points
    return rsquare
}

#[test]
fn testrsquare(){
    let testx=vec![3.0,8.0,10.0,17.0,24.0,27.0];
    let testy=vec![2.0,8.0,10.0,13.0,18.0,20.0];
    //base on online calculator for r square, the value of r square for this test data is 0.9729
    let testrsquarevalue=rsquare(&testx,&testy);
    assert_eq!(testrsquarevalue,0.9729,"The calculation for rsquare is wrong.")
}
