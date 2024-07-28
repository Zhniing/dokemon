extends Node


class_name Hero


var lv
var stats = []
var hp  # Current HP


func _init(_lv = 50, base = [1, 1, 1, 1, 1, 1]):
	# STAT
	lv = _lv
	var iv = [31, 31, 31, 31, 31, 31]
	var ev = [252, 0, 0, 0, 0, 0]
	for i in base.size():
		stats.append(calc_stat(lv, base[i], iv[i], ev[i]))
	stats[0] += 5 + lv  # Maximum HP
	hp = stats[0]


func calc_stat(lv, base, iv, ev):
	return (base * 2 + iv + ev / 4) * lv / 100 + 5
