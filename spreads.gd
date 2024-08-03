class_name Spread
extends Control


var ivs: Array = [31, 31, 31, 31, 31, 31]
var evs: Array = [252, 0, 0, 0, 0, 0]


func calc_stat(lv, base, iv, ev):
	return (base * 2 + iv + ev / 4) * lv / 100 + 5


func calc_stats(base_stats):
	var stats = []
	var lv = get_parent().get_parent().lv
	for i in base_stats.size():
		stats.append(calc_stat(lv, base_stats[i], ivs[i], evs[i]))
	stats[0] += 5 + lv  # HP stat
	return stats
