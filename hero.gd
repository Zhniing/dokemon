class_name Hero
extends Control


var lv = 50
@export var target: Hero


func _process(delta):
	if $HP.hp <= 0:
		get_parent().alive_num -= 1


func damage(x):
	$HP.damage(x)


func revive():
	$HP.heal(get_stat(0))


func get_stat(i):
	return $Stats.stats[i]
