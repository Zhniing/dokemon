extends Control


var alive_num = 1


func _process(delta):
	# Update the alive_num
	var cnt = 0
	for i in get_children():
		if i.get_node("HP").hp > 0:
			cnt += 1
	alive_num = cnt
