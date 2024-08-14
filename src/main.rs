use catenary_tdx_data::*;
use core::result::Result;
//use iso8601::datetime;
use reqwest::{header::AUTHORIZATION, *};
//use serde_json::*;
use std::{collections::HashMap, env, error::Error, fs::File, path::Path};
use std::io::Write;
use catenary_tdx_data::auth::URL_HEAD;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let bus = [
        //static bus by city
        "/v2/Bus/Route/City/city",
        "/v2/Bus/Stop/City/city",
        "/v2/Bus/Operator/City/city",
        "/v2/Bus/Schedule/City/city", //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/City/city",
        "/v2/Bus/Shape/City/city",
        "/v2/Bus/RouteFare/City/city",
        //rt bus by city
        "/v2/Bus/RealTimeByFrequency/City/city",
        "/v2/Bus/RealTimeNearStop/City/city",
        "/v2/Bus/EstimatedTimeOfArrival/City/city",
        "/v2/Bus/Alert/City/city",
    ];
    let ic_bus = [
        //static intercity bus
        "/v2/Bus/Route/InterCity",
        "/v2/Bus/Stop/InterCity",
        "/v2/Bus/Operator/InterCity",
        "/v2/Bus/Schedule/InterCity", //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/InterCity",
        "/v2/Bus/Shape/InterCity",
        "/v2/Bus/RouteFare/InterCity",
        //rt intercity bus
        "/v2/Bus/RealTimeByFrequency/InterCity",
        "/v2/Bus/RealTimeNearStop/InterCity",
        "/v2/Bus/EstimatedTimeOfArrival/InterCity",
        "/v2/Bus/Alert/InterCity",
    ];
    let metro = [
        //static metro
        "/v2/Rail/Metro/Station/metrosystem",
        "/v2/Rail/Metro/Route/metrosystem",
        "/v2/Rail/Metro/FirstLastTimetable/metrosystem",
        "/v2/Rail/Metro/Frequency/metrosystem",
        "/v2/Rail/Metro/Shape/metrosystem",
        "/v2/Rail/Metro/ODFare/metrosystem",
        //rt metro
        "/v2/Rail/Metro/LiveBoard/metrosystem",
        "/v2/Rail/Metro/StationTimeTable/metrosystem",
        "/v2/Rail/Metro/Alert/metrosystem",
    ];
    let rail = [
        //static rail
        "/v2/Rail/Operator",              //also for metro
        "/v2/Rail/THSR/Station",          //theres only one line so they dont have routes
        "/v2/Rail/THSR/GeneralTimetable", //calender, trips, stop times
        "/v2/Rail/THSR/Shape",
        "/v2/Rail/THSR/ODFare",
        "/v3/Rail/TRA/Operator",
        "/v3/Rail/TRA/Station",
        "/v3/Rail/TRA/GeneralTrainTimetable", //calender, trips, stop times
        "/v3/Rail/TRA/Shape",
        "/v3/Rail/TRA/ODFare",
        "/v3/Rail/AFR/Operator",
        "/v3/Rail/AFR/Station",
        "/v3/Rail/AFR/Route",
        "/v3/Rail/AFR/GeneralTrainTimetable",
        "/v3/Rail/AFR/Shape",
        "/v3/Rail/AFR/ODFare",
        //rt rail
        "/v3/Rail/TRA/TrainLiveBoard",
        "/v3/Rail/TRA/StationLiveBoard",
        "/v3/Rail/TRA/Alert",
        "/v2/Rail/THSR/AlertInfo",
    ];
    let city = [
        "Taipei",
        "NewTaipei",
        "Taoyuan",
        "Taichung",
        "Tainan", //also in v3 dataset, might add it as v3? idk if its repeat
        "Kaohsiung",
        "Keelung",
        "Hsinchu",
        "HsinchuCounty",
        "MiaoliCounty",
        "ChanghuaCounty",
        "NantouCounty",
        "YunlinCounty",
        "ChiayiCounty",
        "Chiayi",
        "PingtungCounty",
        "YilanCounty",
        "HualienCounty",
        "TaitungCounty",
        "KinmenCounty",
        "PenghuCounty",
        "LienchiangCounty",
    ];
    let metrosystem = [
        "TRTC", //has live
        "KRTC", //has live
        "KLRT", //has live
        "TYMC", "TRTCMG", //gondola :D
        "TMRT", "NTMC", "NTALRT",
    ];

    let file_path = Path::new("./tdx-secret.json");
    let file = File::open(file_path).expect("file not found");
    let secret: HashMap<String, String> =
        serde_json::from_reader(file).expect("error while reading");

    let client = Client::new();

    let token = catenary_tdx_data::auth::get_token_header(
        secret.get("client_id").unwrap(),
        secret.get("client_secret").unwrap(),
    )
    .await?;

    for loc in city.iter() {
        for elem in &bus {
            let res = fetch(&elem.replace("city", loc), &token, &client)
                .await?
                .text()
                .await?;

            let _s: Vec<&str> = elem.split('/').collect();
            let filename = format!("./output/{}Bus{}.json", loc, _s[3]);
            let mut output = File::create(filename)?;
            output.write_all(&res.as_bytes())?;
        }
        break;
    }

    Ok(())
}

async fn fetch(
    endpoint: &str,
    token: &String,
    client: &Client,
) -> Result<Response, Box<dyn Error + Send + Sync>> {
    let query_url = format!("{}{}", URL_HEAD, endpoint);
    println!("{}\t", query_url);
    thread::sleep(Duration::from_secs(5));

    Ok(client
        .get(&query_url)
        //.header(AUTHORIZATION, token)
        .header("accept", "application/json")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.122 Safari/537.36")
        .send()
        .await?)
}

