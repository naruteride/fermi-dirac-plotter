use std::fmt::Write;
use std::fs;

fn main() -> std::io::Result<()> {
	let ef: f64 = 1.0;				// eV
	let temperature: f64 = 300.0;	// K
	let kb: f64 = 8.617_333_262e-5;	// eV/K
	let kt = kb * temperature;

	let e_min = 0.90;
	let e_max = 1.15;
	let f_max = 2.5;

	let left = 80.0;
	let top = 60.0;
	let width = 680.0;
	let height = 400.0;

	let x = |e: f64| left + (e - e_min) / (e_max - e_min) * width;
	let y = |f: f64| top + (f_max - f) / f_max * height;

	let mut svg = format!(
		r##"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="530">
		<rect width="100%" height="100%" fill="white"/>
		<defs>
			<clipPath id="plot">
				<rect x="{left}" y="{top}" width="{width}" height="{height}"/>
			</clipPath>
		</defs>

		<g fill="none" stroke="black" stroke-width="1">
			<line x1="{left}" y1="{top}" x2="{left}" y2="{}"/>
			<line x1="{left}" y1="{}" x2="{}" y2="{}"/>
		</g>

		<g font-family="sans-serif" font-size="16">
			<text x="80" y="30">E_F = 1 eV, T = 300 K</text>
			<text x="390" y="510">Energy E (eV)</text>
			<text x="30" y="55">f(E)</text>
		</g>
		"##,
		top + height,
		top + height,
		left + width,
		top + height,
	);

	// 가로축
	for i in 0..=5 {
		let e = e_min + i as f64 * 0.05;
		let px = x(e);
		let bottom = top + height;

		write!(
			svg,
			r##"<line x1="{px}" y1="{bottom}" x2="{px}" y2="{}" stroke="black"/>
			<text x="{px}" y="{}" text-anchor="middle"
				font-family="sans-serif" font-size="14">{e:.2}</text>"##,
			bottom + 5.0,
			bottom + 22.0,
		)
		.unwrap();
	}

	// 세로축
	for i in 0..=5 {
		let f = i as f64 * 0.5;
		let py = y(f);

		write!(
			svg,
			r##"<line x1="{}" y1="{py}" x2="{left}" y2="{py}" stroke="black"/>
			<text x="{}" y="{}" text-anchor="end"
				font-family="sans-serif" font-size="14">{f:.1}</text>"##,
			left - 5.0,
			left - 10.0,
			py + 5.0,
		)
		.unwrap();
	}

	let mut fermi_points = String::new();
	let mut boltzmann_points = String::new();

	for i in 0..=1000 {
		let e = e_min + (e_max - e_min) * i as f64 / 1000.0;

		let fermi = 1.0 / (1.0 + ((e - ef) / kt).exp());
		let boltzmann = (-(e - ef) / kt).exp();

		write!(fermi_points, "{:.2},{:.2} ", x(e), y(fermi)).unwrap();
		write!(boltzmann_points, "{:.2},{:.2} ", x(e), y(boltzmann)).unwrap();
	}

	write!(
		svg,
		r##"
		<g clip-path="url(#plot)" fill="none" stroke="black" stroke-width="2">
			<polyline points="{fermi_points}"/>
			<polyline points="{boltzmann_points}" stroke="red"/>
		</g>

		<g fill="none" stroke-width="2">
			<line x1="340" y1="25" x2="370" y2="25" stroke="black"/>
			<line x1="535" y1="25" x2="565" y2="25" stroke="red"/>
		</g>

		<g font-family="sans-serif" font-size="14">
			<text x="380" y="30">Fermi-Dirac</text>
			<text x="575" y="30">Boltzmann</text>
		</g>
		</svg>
		"##
	)
	.unwrap();

	fs::write("fermi_function.svg", svg)?;
	println!("fermi_function.svg 생성 완료");

	Ok(())
}