use plot::{spark, Bar, Orientation, Line, Candle};
use plot::candle::OhlcBar;

fn main() {
    // Sparkline
    println!("Sparkline:");
    println!("  {}", spark(&[1.0, 3.0, 7.0, 2.0, 5.0, 8.0, 4.0, 6.0, 9.0, 3.0], Some(82)));
    println!();

    // Horizontal bar chart
    println!("Horizontal Bar Chart:");
    let mut bar = Bar::new(50, Orientation::Horizontal);
    bar.add("Rust", 95.0, Some(208));
    bar.add("Go", 78.0, Some(81));
    bar.add("Python", 85.0, Some(226));
    bar.add("Ruby", 72.0, Some(196));
    bar.add("C", 90.0, Some(82));
    println!("{}", bar.render());
    println!();

    // Vertical bar chart
    println!("Vertical Bar Chart:");
    let mut vbar = Bar::new(30, Orientation::Vertical);
    vbar.set_height(10);
    vbar.add("Q1", 80.0, Some(82));
    vbar.add("Q2", 95.0, Some(82));
    vbar.add("Q3", 60.0, Some(196));
    vbar.add("Q4", 110.0, Some(82));
    println!("{}", vbar.render());
    println!();

    // Line chart
    println!("Line Chart:");
    let mut line = Line::new(60, 15);
    line.add(&[10.0, 15.0, 12.0, 20.0, 18.0, 25.0, 22.0, 30.0, 28.0, 35.0], 82, Some("Price"));
    line.add(&[5.0, 8.0, 10.0, 7.0, 12.0, 9.0, 15.0, 11.0, 18.0, 14.0], 226, Some("Volume"));
    println!("{}", line.render());
    println!();

    // Candlestick chart
    println!("Candlestick Chart:");
    let mut candle = Candle::new(50, 15);
    candle.add(&[
        OhlcBar { o: 100.0, h: 105.0, l: 98.0, c: 103.0 },
        OhlcBar { o: 103.0, h: 107.0, l: 101.0, c: 99.0 },
        OhlcBar { o: 99.0, h: 104.0, l: 97.0, c: 102.0 },
        OhlcBar { o: 102.0, h: 110.0, l: 100.0, c: 108.0 },
        OhlcBar { o: 108.0, h: 112.0, l: 106.0, c: 105.0 },
        OhlcBar { o: 105.0, h: 109.0, l: 103.0, c: 107.0 },
        OhlcBar { o: 107.0, h: 115.0, l: 105.0, c: 113.0 },
        OhlcBar { o: 113.0, h: 116.0, l: 110.0, c: 111.0 },
        OhlcBar { o: 111.0, h: 118.0, l: 109.0, c: 117.0 },
        OhlcBar { o: 117.0, h: 120.0, l: 114.0, c: 119.0 },
    ]);
    println!("{}", candle.render());
}
