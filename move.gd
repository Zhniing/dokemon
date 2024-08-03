extends Control


var move_name = "attack"
var power = 90

var source
var target
var lv
var att
var def


func _ready():
	source = get_parent().get_parent()  # Hero
	lv = source.lv
	target = source.target


func calc_damage(lv, att, def, power):
	return (2 * lv + 10) / 250.0 * (float(att) / def) * power + 2


func attack():
	# Survival check
	if source.get_node("HP").hp <= 0 || target.get_node("HP").hp <= 0:
		return

	att = source.get_stat(3)
	def = target.get_stat(4)

	# Deal the damage
	var damage = int(calc_damage(lv, att, def, power))
	target.damage(damage)

	# Update the battle log
	var log = $/root/Arena/BattleLog
	log.text += "\t" + source.get_parent().name + " 的 " + source.name + " 对 " + target.get_parent().name + " 的 " + target.name + " 使用 " + move_name + " 造成 " + str(damage) + " 点伤害" + "\n"
	if target.get_node("HP").hp <= 0:
		log.text += "\t" + target.get_parent().name + " 的 " + target.name + " 倒下了" + "\n"
