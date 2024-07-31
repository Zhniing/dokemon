extends Control


func calc_damage(lv, att, def, power):
	return (2 * lv + 10) / 250.0 * (float(att) / def) * power + 2


func attack(s, t):
	# Survival check
	if s.hero.hp <= 0 || t.hero.hp <= 0:
		return

	var lv = s.hero.lv
	var att = s.hero.stats[3]
	var def = t.hero.stats[4]
	var power = 90

	# Deal the damage
	var damage = int(calc_damage(lv, att, def, power))
	t.hero.hp -= damage

	# Update the battle log
	var log = $/root/Arena/BattleLog
	log.text += s.name + " 对 " + t.name + " 使用了 " + "attack" + " 造成了 " + str(damage) + " 点伤害" + "\n"
	if t.hero.hp <= 0:
		log.text += t.name + " 倒下了" + "\n"

	# Update the UI
	t.get_node("HP").value = t.hero.hp
