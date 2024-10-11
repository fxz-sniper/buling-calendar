use std::{collections::HashMap, str::FromStr};

use chrono::{Datelike, Days, Local, NaiveDate};
use egui::{CentralPanel, Color32, RichText, ScrollArea, TopBottomPanel, Ui};

use crate::holiday::{get_holidays, HolidayData};

const WEEK_DATA: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const WEEK_NUM: u32 = 7;

pub struct MyApp {
    pub(crate) holiday_data: HolidayData,
    year: i32,
    month: u32,
    dark_but_flag: bool,
}

impl MyApp {
    pub fn new(holiday_data: HolidayData) -> Self {
        let now = Local::now();
        let year = now.year();
        let month = now.month();
        Self {
            holiday_data,
            year,
            month,
            dark_but_flag: true,
        }
    }

    fn theme_switcher(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if !self.dark_but_flag {
            if ui.button("Light").clicked() {
                ctx.set_visuals(egui::Visuals::light());
                self.dark_but_flag = true;
            }
        } else {
            if ui.button("Dark").clicked() {
                ctx.set_visuals(egui::Visuals::dark());
                self.dark_but_flag = false;
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (holidays, workdays) =
            get_holidays_and_workday_in_month(&self.holiday_data, self.year, self.month);

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                create_calendar_ui(ui, self, ctx, holidays, workdays);
            });
        });
    }
}

fn create_change_button(my_app: &mut MyApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.add_space(30.0);
        ui.spacing_mut().item_spacing = egui::vec2(10.0, 20.0);

        if ui.button("Month +1").clicked() {
            // month +1
            if my_app.month == 12 {
                my_app.year += 1;
                my_app.month = 1;

                match get_holidays(my_app.year) {
                    Ok(holiday_data) => {
                        my_app.holiday_data = holiday_data;
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                        ui.label(RichText::new("failed to get holidays info").color(Color32::RED));
                    }
                }
            } else {
                my_app.month += 1;
            }
        }

        if ui.button("Month -1").clicked() {
            // month -1
            if my_app.month == 1 {
                my_app.year -= 1;
                my_app.month = 1;

                match get_holidays(my_app.year) {
                    Ok(holiday_data) => {
                        my_app.holiday_data = holiday_data;
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                        ui.label(RichText::new("failed to get holidays info").color(Color32::RED));
                    }
                }
            } else {
                my_app.month -= 1;
            }
        }
    });
}

pub fn create_calendar_ui(
    ui: &mut Ui,
    my_app: &mut MyApp,
    ctx: &egui::Context,
    holidays: HashMap<u32, String>,
    workdays: HashMap<u32, String>,
) {
    let days = get_days_of_month(my_app.year, my_app.month);

    let first_day_date = NaiveDate::from_ymd_opt(my_app.year, my_app.month, 1).unwrap();
    let first_day_weekday_str = first_day_date.weekday().to_string();

    let mut count = -1;

    let mut current_date = first_day_date.clone();

    TopBottomPanel::top("header_panel1").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(30.0);
            ui.heading(&format!("{} - {} ", my_app.year, my_app.month));
            // dark or light mode
            my_app.theme_switcher(ui, ctx);
        });
        ui.add_space(5.0);
        create_change_button(my_app, ui);
        ui.add_space(10.0);
    });

    TopBottomPanel::top("header_panel2").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            // label headings and find index of first weekday
            for (idx, ele) in WEEK_DATA.iter().enumerate() {
                ui.label(*ele);

                if &first_day_weekday_str == *ele {
                    count = idx as i32;
                }
            }
        });
    });

    // divide into segments each week
    let p_flag = (days + count as u32) / WEEK_NUM;
    let mut periods = Vec::new();
    for i in 1..=(p_flag + 1) {
        if i == 1 {
            periods.push(WEEK_NUM - count as u32);
        } else if i > 1 && i < (p_flag + 1) {
            periods.push(WEEK_NUM);
        } else {
            periods.push(days - (p_flag * WEEK_NUM - count as u32));
        }
    }

    CentralPanel::default().show_inside(ui, |ui| {
        for (idx, p) in periods.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(15.0, 20.0);
                if idx == 0 && count > 0 {
                    for _i in 0..count {
                        ui.label("00");
                    }
                }
                for date in current_date.iter_days().take((*p).try_into().unwrap()) {
                    let num_str = date.day().to_string();
                    let mut text = String::new();
                    if num_str.len() == 1 {
                        text = format!("0{}", num_str)
                    } else if num_str.len() == 2 {
                        text = num_str;
                    }

                    if holidays.contains_key(&date.day())
                        || (date.weekday().to_string() == WEEK_DATA[0]
                            || date.weekday().to_string() == WEEK_DATA[6]
                                && !workdays.contains_key(&date.day()))
                    {
                        ui.label(RichText::new(text).color(Color32::RED));
                    } else {
                        ui.label(text);
                    }
                }
            });
            current_date = current_date.checked_add_days(Days::new(*p as u64)).unwrap();
        }

        ui.add_space(20.0);
    });
}

fn get_days_of_month(year: i32, month: u32) -> u32 {
    match month {
        2 if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/* fn get_weeks_days_of_month(year: i32, month: u32) -> (u32, usize){
    let days = get_days_of_month(year, month);
    let first_day_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let end_day_date = NaiveDate::from_ymd_opt(year, month, days).unwrap();

    let mut weeks = end_day_date.signed_duration_since(first_day_date).num_weeks() as usize;
    if days % 7 != 0 {
        weeks += 1;
    }
    (days, weeks)
} */

fn get_holidays_and_workday_in_month(
    holiday_data: &HolidayData,
    year: i32,
    month: u32,
) -> (HashMap<u32, String>, HashMap<u32, String>) {
    let days = get_days_of_month(year, month);
    let first_day_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let end_day_date = NaiveDate::from_ymd_opt(year, month, days).unwrap();

    let mut holidays = HashMap::<u32, String>::new();
    let mut workdays = HashMap::<u32, String>::new();
    if !holiday_data.holiday.is_empty() {
        for holiday in holiday_data.holiday.values().into_iter() {
            if holiday.date >= first_day_date.to_string()
                && holiday.date <= end_day_date.to_string()
            {
                let date = NaiveDate::from_str(&holiday.date).unwrap();
                if holiday.holiday {
                    holidays.insert(date.day(), holiday.date.to_string());
                } else {
                    workdays.insert(date.day(), holiday.date.to_string());
                }
            }
        }
    }
    (holidays, workdays)
}
