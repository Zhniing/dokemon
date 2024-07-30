extends Control


var base = [85, 60, 65, 130, 75, 98]
var hero = Hero.new(50, base)

func _ready():
	if name == "Player2":
		hero.stats[5] += 1  # TEST
