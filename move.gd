extends Control


func calc_damage(lv, att, def, power):
	return (2 * lv + 10) / 250.0 * (float(att) / def) * power + 2


func attack(s, t):
	var lv = s.hero.lv
	var att = s.hero.stats[3]
	var def = t.hero.stats[4]
	var power = 90

	# Deal the damage
	var damage = calc_damage(lv, att, def, power)
	t.hero.hp -= damage

	# Update the UI
	t.get_node("HP").value = t.hero.hp
