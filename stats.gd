extends Control


var base_stats: Array = [85, 60, 65, 130, 75, 98]
var stats: Array


func _ready():
	if get_parent().get_parent().name == "Player2":
		base_stats[5] += 1  # TEST

	var spread = $Spread
	if spread:
		stats = spread.calc_stats(base_stats)
