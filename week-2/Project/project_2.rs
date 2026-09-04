fn main() {
	let toshiba_quantity = 2;
	let toshiba_amount = 450_000;
	let toshiba_total = toshiba_quantity * toshiba_amount;

	let mac_quantity = 1;
	let mac_amount = 1_500_000;
	let mac_total = mac_quantity * mac_amount; 

	let hp_quantity = 3;
	let hp_amount = 750_000;
	let hp_total = hp_quantity * hp_amount;

	let dell_quantity = 3;
	let dell_amount = 2_850_000;
	let dell_total = dell_quantity * dell_amount;

	let acer_quantity = 1;
	let acer_amount = 250_000;
	let acer_total = acer_quantity * acer_amount;

	let total_quantity = toshiba_quantity + mac_quantity + hp_quantity + dell_quantity + acer_quantity;
	let total_sum = toshiba_total + mac_total + hp_total + dell_total + acer_total;

	let average = total_sum / total_quantity;

	println! ("The total sum is {} ", total_sum);
	println! ("The average is {}", average);
}