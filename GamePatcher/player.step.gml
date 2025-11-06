var bJump, leftX, leftY, rightX, rightY, f, _id, _id2, Lon, Ron;
hspeed = 0
e_read_input()
if obj_gameMgr.i_l
{
	if (obj_gameMgr.playinput == 1)
	{
		if place_free((x - 3), y)
		{
			if (obj_gameMgr.pushBlockL == 0)
			{
				if (instance_exists(obj_magicBombSlow) == 0)
				{
					hspeed = (-pHSPEED)
					dir = 1
				}
				else
				{
					hspeed = -1
					dir = 1
				}
			}
			if (obj_gameMgr.pushBlockL == 1)
			{
				hspeed = -1
				dir = 1
			}
			if (obj_gameMgr.pushBlockBlueL == 1)
			{
				hspeed = -1
				dir = 1
			}
			if (obj_gameMgr.pushBlockGreenL == 1)
			{
				hspeed = -1
				dir = 1
			}
			if (obj_gameMgr.pushBlockYellowL == 1)
			{
				hspeed = -1
				dir = 1
			}
		}
		else
			move_contact_solid(180, 4)
	}
}
if obj_gameMgr.i_r
{
	if (obj_gameMgr.playinput == 1)
	{
		if place_free((x + 3), y)
		{
			if (obj_gameMgr.pushBlockR == 0)
			{
				if (instance_exists(obj_magicBombSlow) == 0)
				{
					hspeed = pHSPEED
					dir = 0
				}
				else
				{
					hspeed = 1
					dir = 0
				}
			}
			if (obj_gameMgr.pushBlockR == 1)
			{
				hspeed = 1
				dir = 0
			}
			if (obj_gameMgr.pushBlockBlueR == 1)
			{
				hspeed = 1
				dir = 0
			}
			if (obj_gameMgr.pushBlockGreenR == 1)
			{
				hspeed = 1
				dir = 0
			}
			if (obj_gameMgr.pushBlockYellowR == 1)
			{
				hspeed = 1
				dir = 0
			}
		}
		else
			move_contact_solid(0, 4)
	}
}
if obj_gameMgr.i_j
{
	if (obj_gameMgr.playinput == 1)
	{
		if (global.character == "sekibanki")
		{
			if (vspeed == 0)
			{
				bJump = 0
				if (place_free(x, (y + 1)) == 0)
					bJump = 1
				if place_meeting(x, (y + 1), obj_floor)
					bJump = 1
				if bJump
				{
					vspeed = -4.7
					gravity = 0.3
					audio_play_sound(se_jump, 10, false)
					instance_create(obj_player.x, obj_player.y, obj_jumpEffect)
				}
			}
		}
		else if (global.character == "rumia")
		{
			if (floorCheck == 0)
			{
				vspeed = -4.3
				gravity = 0.3
				audio_play_sound(se_jump, 10, false)
				instance_create(obj_player.x, obj_player.y, obj_jumpEffect)
				floorCheck += 1
			}
			else if (floorCheck == 1)
			{
				vspeed = -6
				gravity = 0.3
				audio_play_sound(se_jump2, 10, false)
				instance_create(obj_player.x, obj_player.y, obj_jumpEffect)
				floorCheck += 1
			}
		}
		else if (global.character == "cirno")
		{
			if (vspeed == 0)
			{
				bJump = 0
				if (place_free(x, (y + 1)) == 0)
					bJump = 1
				if place_meeting(x, (y + 1), obj_floor)
					bJump = 1
				if bJump
				{
					vspeed = -6.5
					gravity = 0.3
					audio_play_sound(se_jump, 10, false)
					instance_create(obj_player.x, obj_player.y, obj_jumpEffect)
				}
			}
		}
		if (global.character == "seija")
		{
			if (vspeed == 0)
			{
				bJump = 0
				if (place_free(x, (y + 1)) == 0)
					bJump = 1
				if place_meeting(x, (y + 1), obj_floor)
					bJump = 1
				if bJump
				{
					vspeed = -4.7
					gravity = 0.3
					audio_play_sound(se_jump, 10, false)
					instance_create(obj_player.x, obj_player.y, obj_jumpEffect)
				}
			}
		}
	}
}
if (vspeed > 6)
	vspeed = 6
if !place_free(x, (y + 1))
	gravity = 0
else if !instance_exists(obj_e_precipitator)
	gravity = 0.3
else {
	_id = collision_rectangle(bbox_left, bbox_bottom - 1, bbox_right, bbox_bottom + 1, obj_floor, true, true)
	if (_id != noone && _id.y > bbox_bottom - 1 && vspeed >= 0) {
		floorCheck = 0
		gravity = 0
	} else
		gravity = 0.3
}
if (hspeed == 0 && vspeed == 0)
	state = 0
else if (hspeed != 0 && vspeed == 0)
	state = 1
else if (vspeed < 0)
	state = 2
else if (vspeed > 0)
	state = 3
tAnim++
switch state
{
	case 0:
		subImage = 0
		if ((tAnim % 120) > 110)
			subImage = 1
		break
	case 1:
		subImage = 2
		if ((tAnim % 12) < 6)
			subImage = 3
		if ((tAnim % 12) == 0)
			instance_create(obj_player.x, obj_player.y, obj_walkEffect)
		break
	case 2:
		subImage = 4
		break
	case 3:
		subImage = 5
		break
	case 4:
		subImage = 5
		if ((tAnim % 4) < 2)
			subImage = 6
		break
	case 5:
		subImage = 5
		break
	case 6:
		subImage = 6
		break
}

if (global.character == "sekibanki")
{
	if (headHold == 0)
	{
		if obj_gameMgr.i_z
		{
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.bankiHead > 0)
			{
				if (dir == 1)
				{
					obj_gameMgr.bankiHead--
					audio_play_sound(se_head, 10, false)
					with (instance_create((obj_player.x + 1), (obj_player.y -6), obj_head))
					{
						if (global.bankiSprite == 16)
							subImage = 11
						else
							subImage = irandom(8) + 2
					}
					new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
				}
				else
				{
					obj_gameMgr.bankiHead--
					audio_play_sound(se_head, 10, false)
					with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head))
					{
						if (global.bankiSprite == 16)
							subImage = 11
						else
							subImage = irandom(8) + 2
					}
					new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
				}
				global.play_head++
			}
			else if (obj_gameMgr.playinput == 1 && obj_gameMgr.bankiHead == 0)
				audio_play_sound(se_not, 10, false)
		}
	}
	else if (headHold == 1)
	{
		if obj_gameMgr.i_z
		{
			if (obj_gameMgr.playinput == 1)
			{
				if (dir == 1)
				{
					audio_play_sound(se_head, 10, false)
					if (obj_holdHead.spring >= 1)
					{
						if (obj_holdHead.spring == 1)
						{
							with (instance_create((obj_player.x + 1), (obj_player.y -6), obj_head))
								spring = 1
						}
						else if (obj_holdHead.spring == 2)
						{
							with (instance_create((obj_player.x + 1), (obj_player.y -6), obj_head))
								spring = 2
						}
						else if (obj_holdHead.spring == 3)
						{
							with (instance_create((obj_player.x + 1), (obj_player.y -6), obj_head))
							{
								spring = 3
								global.sukimaBall1 = id
							}
						}
						else if (obj_holdHead.spring == 4)
						{
							with (instance_create((obj_player.x + 1), (obj_player.y -6), obj_head))
							{
								spring = 4
								global.sukimaBall2 = id
							}
						}
					}
					else
					{
						with (instance_create((obj_player.x + 1), (obj_player.y -6), obj_head))
						{
							if (global.bankiSprite == 16)
								subImage = 11
							else
								subImage = irandom(8) + 2
						}
					}
					new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
					instance_destroy(obj_holdHead)
					headHold = 0
				}
				else
				{
					audio_play_sound(se_head, 10, false)
					if (obj_holdHead.spring >= 1)
					{
						if (obj_holdHead.spring == 1)
						{
							with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head))
								spring = 1
						}
						else if (obj_holdHead.spring == 2)
						{
							with (instance_create((obj_player.x - 1), (obj_player.y + 6), obj_head))
								spring = 2
						}
						else if (obj_holdHead.spring == 3)
						{
							with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head))
							{
								spring = 3
								global.sukimaBall1 = id
							}
						}
						else if (obj_holdHead.spring == 4)
						{
							with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head))
							{
								spring = 4
								global.sukimaBall2 = id
							}
						}
					}
					else
					{
						with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head))
						{
							if (global.bankiSprite == 16)
								subImage = 11
							else
								subImage = irandom(8) + 2
						}
					}
					new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
					instance_destroy(obj_holdHead)
					headHold = 0
				}
			}
			else if (obj_gameMgr.playinput == 1 && obj_gameMgr.bankiHead == 0)
				audio_play_sound(se_not, 10, false)
		}
	}
	if obj_gameMgr.i_x
	{
		if (obj_gameMgr.playinput == 1 && obj_gameMgr.bankiHead > 0)
		{
			if obj_gameMgr.i_u
			{
				obj_gameMgr.bankiHead--
				audio_play_sound(se_head, 10, false)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_headThrowUp)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
			}
			else if (dir == 1)
			{
				obj_gameMgr.bankiHead--
				audio_play_sound(se_head, 10, false)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_headThrow)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
			}
			else
			{
				obj_gameMgr.bankiHead--
				audio_play_sound(se_head, 10, false)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_headThrow)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
			}
			global.play_head++
		}
		else if (obj_gameMgr.playinput == 1 && obj_gameMgr.bankiHead == 0)
			audio_play_sound(se_not, 10, false)
	}
}
if (global.character == "rumia")
{
	if obj_gameMgr.i_z
	{
		if (obj_gameMgr.playinput == 1 && headHold) {
			audio_play_sound(se_head, 10, false)
			with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head)) {
				if (other.dir)
					x += 2
				spring = obj_holdHead.spring
				if (spring == 3)
					global.sukimaBall1 = id
				else if (spring == 4)
					global.sukimaBall2 = id
			}
			instance_create(obj_player.x, obj_player.y, obj_headEffect)
			instance_destroy(obj_holdHead)
			headHold = false
		}

		else if (obj_gameMgr.playinput == 1 && obj_gameMgr.energy > 0)
		{
			obj_gameMgr.energy -= 1
			instance_create(x, y, obj_darkBomb)
			instance_create(x, y, obj_headEffect)
			audio_play_sound(se_dark, 10, false)
		}
		else if (obj_gameMgr.playinput == 1 && obj_gameMgr.energy == 0)
			audio_play_sound(se_not, 10, false)
	}
	darkBlockTimer++
	if obj_gameMgr.i_x
	{
		if (obj_gameMgr.playinput == 1 && obj_gameMgr.energy > 0 && darkBlockFlag == 0)
		{
			if (dir == 1)
			{
				leftX = floor(obj_player.x / 32) * 32
				leftY = floor(obj_player.y / 32) * 32
				obj_gameMgr.energy -= 1
				instance_create(leftX - 32, leftY, obj_darkBlock)
				instance_create(x, y, obj_darkBlockEffectL)
				audio_play_sound(se_dark, 10, false)
			}
			else if (dir == 0)
			{
				rightX = floor(obj_player.x / 32) * 32
				rightY = floor(obj_player.y / 32) * 32
				obj_gameMgr.energy -= 1
				instance_create(rightX + 64, rightY, obj_darkBlock)
				instance_create(x, y, obj_darkBlockEffect)
				audio_play_sound(se_dark, 10, false)
			}
			darkBlockTimer = 100
			darkBlockFlag = 1
			obj_gameMgr.playinput = 0
			FreezePoint = obj_player.x
			if instance_exists(obj_rumiaOther)
			{
				obj_rumiaOther.visible = true
				obj_player.visible = false
			}
			else
			{
				instance_create(x, y, obj_rumiaOther)
				obj_rumiaOther.visible = true
				obj_player.visible = false
			}
		}
		else if (obj_gameMgr.playinput == 1 && obj_gameMgr.energy == 0 && darkBlockFlag == 0)
			audio_play_sound(se_not, 10, false)
	}
	if (darkBlockFlag == 1 && !instance_exists(obj_e_playerBonkTimer))
	{
		if (dir == 0)
		{
			if place_free((x - 3), y)
				x += ((FreezePoint - 32 - x) * 0.1)
		}
		else if place_free((x + 3), y)
			x += ((FreezePoint + 32 - x) * 0.1)
	}
	if (darkBlockTimer == 120)
	{
		darkBlockFlag = 0
		obj_gameMgr.playinput = 1
		if instance_exists(obj_rumiaOther)
		{
			obj_rumiaOther.visible = false
			obj_player.visible = true
		}
	}
	if obj_gameMgr.i_c
	{
		if (obj_gameMgr.playinput == 1 && (headHold || collision_rectangle(bbox_left, bbox_top, bbox_right, bbox_bottom, obj_head, true, true) == noone))
		{
			if (instance_exists(obj_darkBomb) == 1 || instance_exists(obj_darkBlock) == 1)
			{
				if (instance_exists(obj_darkBomb) == 1)
					instance_destroy(obj_darkBomb)
				if (instance_exists(obj_darkBlock) == 1)
					instance_destroy(obj_darkBlock)
				audio_play_sound(se_darkBreak, 10, false)
				obj_gameMgr.energy = obj_gameMgr.darkRefundTo
				obj_gameMgr.darkRefundTo -= 1
			}
			else
				audio_play_sound(se_not, 10, false)
		}
	}
}
if (global.character == "cirno")
{
	if obj_gameMgr.i_z
	{
		if (obj_gameMgr.playinput == 1 && headHold) {
			audio_play_sound(se_head, 10, false)
			with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head)) {
				if (other.dir)
					x += 2
				spring = obj_holdHead.spring
				if (spring == 3)
					global.sukimaBall1 = id
				else if (spring == 4)
					global.sukimaBall2 = id
			}
			instance_create(obj_player.x, obj_player.y, obj_headEffect)
			instance_destroy(obj_holdHead)
			headHold = false
		}

		else if (obj_gameMgr.playinput == 1 && global.iceResidual == 0 && obj_gameMgr.ice > 0)
		{
			global.iceResidual = 1
			if obj_gameMgr.i_u
			{
				obj_gameMgr.ice--
				audio_play_sound(se_iceShot, 10, false)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_iceBulletUp)
				new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
				darkBlockTimer = 100
				darkBlockFlag = 1
				obj_gameMgr.playinput = 0
				if instance_exists(obj_cirnoOther)
				{
					obj_cirnoOther.visible = true
					obj_cirnoOther.subImage = 1
					obj_player.visible = false
				}
				else
				{
					instance_create(x, y, obj_cirnoOther)
					obj_cirnoOther.visible = true
					obj_cirnoOther.subImage = 1
					obj_player.visible = false
				}
			}
			else
			{
				if (dir == 1)
				{
					obj_gameMgr.ice--
					audio_play_sound(se_iceShot, 10, false)
					new_instance = instance_create(obj_player.x, obj_player.y, obj_iceBullet)
					new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
					instance_create(x, y, obj_darkBlockEffectL)
				}
				else
				{
					obj_gameMgr.ice--
					audio_play_sound(se_iceShot, 10, false)
					new_instance = instance_create(obj_player.x, obj_player.y, obj_iceBullet)
					new_instance = instance_create(obj_player.x, obj_player.y, obj_headEffect)
					instance_create(x, y, obj_darkBlockEffect)
				}
				if instance_exists(obj_cirnoOther)
				{
					obj_cirnoOther.visible = true
					obj_cirnoOther.subImage = 0
					obj_player.visible = false
				}
				else
				{
					instance_create(x, y, obj_cirnoOther)
					obj_cirnoOther.visible = true
					obj_cirnoOther.subImage = 0
					obj_player.visible = false
				}
			}
			darkBlockTimer = 100
			darkBlockFlag = 1
			obj_gameMgr.playinput = 0
			FreezePoint = obj_player.x
			if instance_exists(obj_cirnoOther)
			{
				obj_cirnoOther.visible = true
				obj_player.visible = false
			}
			else
			{
				instance_create(x, y, obj_cirnoOther)
				obj_cirnoOther.visible = true
				obj_player.visible = false
			}
		}
		else if (obj_gameMgr.playinput == 1 && global.iceResidual == 0 && obj_gameMgr.ice == 0)
			audio_play_sound(se_not, 10, false)
	}
	if (darkBlockFlag == 1 && !instance_exists(obj_e_playerBonkTimer))
	{
		if (dir == 0)
		{
			if place_free((x - 3), y)
				x += ((FreezePoint - 32 - x) * 0.1)
		}
		else if place_free((x + 3), y)
			x += ((FreezePoint + 32 - x) * 0.1)
	}
	darkBlockTimer++
	if (darkBlockTimer == 120)
	{
		darkBlockFlag = 0
		obj_gameMgr.playinput = 1
		if instance_exists(obj_cirnoOther)
		{
			obj_cirnoOther.visible = false
			obj_player.visible = true
		}
	}
	if obj_gameMgr.i_x
	{
		if (obj_gameMgr.playinput == 1 && instance_exists(obj_wall5))
		{
			instance_destroy(obj_wall5)
			audio_play_sound(se_block, 30, false)
			audio_play_sound(se_darkBreak, 30, false)
		}
		else if (obj_gameMgr.playinput == 1 && (!instance_exists(obj_wall5)))
			audio_play_sound(se_not, 10, false)
	}
}
if (global.character == "seija")
{
	if obj_gameMgr.i_z
	{
		if (obj_gameMgr.playinput == 1 && headHold) {
			audio_play_sound(se_head, 10, false)
			with (instance_create((obj_player.x - 1), (obj_player.y -6), obj_head)) {
				if (other.dir)
					x += 2
				spring = obj_holdHead.spring
				if (spring == 3)
					global.sukimaBall1 = id
				else if (spring == 4)
					global.sukimaBall2 = id
			}
			instance_create(obj_player.x, obj_player.y, obj_headEffect)
			instance_destroy(obj_holdHead)
			headHold = false
		}
		else if (global.seijaItem == 0 && (obj_gameMgr.seijaCan & 1))
		{
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost >= 2 && darkBlockFlag == 0 && instance_exists(obj_seijaUI) == 0)
			{
				instance_create(x, y, obj_seijaHirarinuno)
				global.seijaItemResidual = 1
				obj_gameMgr.cost -= 2
				darkBlockTimer = 0
				darkBlockFlag = 1
				obj_gameMgr.playinput = 0
				FreezePoint = obj_player.x
				if instance_exists(obj_seijaOther)
				{
					obj_seijaOther.subImage = 0
					obj_seijaOther.visible = true
					obj_player.visible = false
				}
				else
				{
					new_instance = instance_create(x, y, obj_seijaOther)
					with (new_instance)
						subImage = 0
					obj_seijaOther.visible = true
					obj_player.visible = false
				}
			}
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost < 2 && instance_exists(obj_seijaHirarinuno) == 0 && instance_exists(obj_seijaUI) == 0)
				audio_play_sound(se_not, 10, false)
		}
		else if (global.seijaItem == 1 && instance_exists(obj_seijaCamera))
		{
			if (global.seijaCamera == 0)
			{
				if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost >= 3 && darkBlockFlag == 0 && instance_exists(obj_seijaUI) == 0)
				{
					instance_create(x, y, obj_seijaCameraShutter)
					if (dir == 1)
					{
						leftX = obj_player.x % 32
						leftY = obj_player.y % 32
						obj_gameMgr.cost -= 3
						instance_create((x - (leftX + 96)), (y + (-leftY) - 32), obj_seijaCameraCut1)
						instance_create((x - (leftX + 64)), (y + (-leftY) - 32), obj_seijaCameraCut2)
						instance_create((x - (leftX + 32)), (y + (-leftY) - 32), obj_seijaCameraCut3)
						instance_create((x - (leftX + 96)), (y + (-leftY)), obj_seijaCameraCut4)
						instance_create((x - (leftX + 64)), (y + (-leftY)), obj_seijaCameraCut5)
						instance_create((x - (leftX + 32)), (y + (-leftY)), obj_seijaCameraCut6)
						audio_play_sound(se_camera, 10, false)
					}
					else if (dir == 0)
					{
						rightX = obj_player.x % 32
						rightY = obj_player.y % 32
						obj_gameMgr.cost -= 3
						instance_create((x + ((-rightX) + 64)), (y + (-rightY) - 32), obj_seijaCameraCut1)
						instance_create((x + ((-rightX) + 96)), (y + (-rightY) - 32), obj_seijaCameraCut2)
						instance_create((x + ((-rightX) + 128)), (y + (-rightY) - 32), obj_seijaCameraCut3)
						instance_create((x + ((-rightX) + 64)), (y + (-rightY)), obj_seijaCameraCut4)
						instance_create((x + ((-rightX) + 96)), (y + (-rightY)), obj_seijaCameraCut5)
						instance_create((x + ((-rightX) + 128)), (y + (-rightY)), obj_seijaCameraCut6)
						audio_play_sound(se_camera, 10, false)
					}
					darkBlockTimer = 100
					darkBlockFlag = 1
					obj_gameMgr.playinput = 0
					FreezePoint = obj_player.x
					global.seijaCamera = 1
					if instance_exists(obj_seijaOther)
					{
						obj_seijaOther.visible = true
						obj_seijaOther.subImage = 1
						obj_player.visible = false
					}
					else
					{
						new_instance = instance_create(x, y, obj_seijaOther)
						with (new_instance)
							subImage = 1
						obj_seijaOther.visible = true
						obj_player.visible = false
					}
				}
			}
			else if (global.seijaCamera == 1 && instance_exists(obj_seijaUI) == 0)
			{
				if (dir == 1)
					scr_seijaCameraCutLeft()
				else if (dir == 0)
					scr_seijaCameraCutRight()
				global.seijaCamera = 0
				audio_play_sound(se_camera2, 10, false)
			}
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost < 3 && instance_exists(obj_seijaCameraCut1) == 0 && instance_exists(obj_seijaUI) == 0)
				audio_play_sound(se_not, 10, false)
		}
		else if (global.seijaItem == 1 && (obj_gameMgr.seijaCan & 2) != 0 && obj_gameMgr.playinput)
		{
			if (collision_rectangle(obj_e_seijaCamera.bbox_left, obj_e_seijaCamera.bbox_top, obj_e_seijaCamera.bbox_right, obj_e_seijaCamera.bbox_bottom, obj_e_noPhotography, true, true) != noone)
				audio_play_sound(se_not, 10, false)
			else if (global.seijaCamera)
				e_sc_paste()
			else if (!darkBlockFlag && !instance_exists(obj_seijaUI)) {
				if (obj_gameMgr.cost >= 3) {
					e_sc_cut()
					obj_gameMgr.cost -= 3
				} else
					audio_play_sound(se_not, 10, false)
			}
		}
		else if (global.seijaItem == 2 && (obj_gameMgr.seijaCan & 4) != 0)
		{
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost >= 3 && instance_exists(obj_magicBomb) == 0 && instance_exists(obj_seijaUI) == 0)
			{
				instance_create(obj_player.x, (obj_player.y - 24), obj_magicBomb)
				global.seijaItemResidual = 1
				obj_gameMgr.cost -= 3
			}
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost < 3 && instance_exists(obj_magicBomb) == 0 && instance_exists(obj_seijaUI) == 0)
				audio_play_sound(se_not, 10, false)
		}
		else if (global.seijaItem == 3 && (obj_gameMgr.seijaCan & 8) != 0)
		{
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost >= 4 && hammerFlag == 0 && instance_exists(obj_seijaUI) == 0)
			{
				instance_create(x, y, obj_hammer)
				obj_gameMgr.cost -= 4
				darkBlockTimer = 60
				hammerFlag = 1
				obj_gameMgr.playinput = 0
				FreezePoint = obj_player.x
				if instance_exists(obj_seijaOther)
				{
					obj_seijaOther.subImage = 2
					obj_seijaOther.visible = true
					obj_player.visible = false
				}
				else
				{
					new_instance = instance_create(x, y, obj_seijaOther)
					with (new_instance)
						subImage = 2
					obj_seijaOther.visible = true
					obj_player.visible = false
				}
			}
			if (obj_gameMgr.playinput == 1 && obj_gameMgr.cost < 4 && instance_exists(obj_seijaUI) == 0)
				audio_play_sound(se_not, 10, false)
		}
		if (obj_gameMgr.playinput == 0 && instance_exists(obj_seijaUI) == 1)
		{
			if (obj_seijaUI.decide == 0)
			{
				obj_seijaUI.decide = 1
				audio_play_sound(se_decideSeija, 30, false)
			}
		}
	}
	darkBlockTimer++
	if (darkBlockTimer == 120)
	{
		darkBlockFlag = 0
		hammerFlag = 0
		obj_gameMgr.playinput = 1
		if instance_exists(obj_seijaOther)
		{
			obj_seijaOther.visible = false
			obj_player.visible = true
		}
	}
	if obj_gameMgr.i_x
	{
		if (obj_gameMgr.playinput == 1 && obj_gameMgr.seijaCan != 0)
		{
			if (instance_exists(obj_seijaUI) == 0)
			{
				if (global.seijaItemResidual == 0)
					instance_create(x, y, obj_seijaUI)
			}
		}
	}
}
floorCheckCount = 0
f = frac(obj_player.y)
if (floorCheckCount == -1)
	pdy = floor(obj_player.y)
if (f <= 0.5)
	pdy = floor(obj_player.y) - 1
else
	pdy = floor(obj_player.y)
_id = collision_line((x + 5), (y + 3), (x + 26), (y + 31), obj_wallRed, true, true)
if (_id != -4)
	blockCollision = 1
else
	blockCollision = 0
_id2 = collision_line((x + 7), (y + 3), (x + 24), (y + 31), obj_wallRedOff, true, true)
if (_id2 != -4)
	blockCollision = 1
else
	blockCollision = 0
Lon = collision_rectangle((x + 6), (y + 30), (x + 25), (y + 33), obj_Lfloor, 1, 1)
if (Lon != -4)
{
	if place_free((x - 1), y)
		x -= 1
	else
		move_contact_solid(180, 1)
}
Ron = collision_rectangle((x + 6), (y + 30), (x + 25), (y + 33), obj_Rfloor, 1, 1)
if (Ron != -4)
{
	if place_free((x + 1), y)
		x += 1
	else
		move_contact_solid(0, 1)
}
if obj_gameMgr.i_u
	cameraTimer++
if obj_gameMgr.i_w_u && !obj_gameMgr.i_u
{
	cameraTimer = 0
	cameraUp = 0
}
if obj_gameMgr.i_d
	cameraTimer--
if obj_gameMgr.i_w_d && !obj_gameMgr.i_d
{
	cameraTimer = 0
	cameraDown = 0
}
if (cameraTimer > 30)
	cameraUp = 1
else if (cameraTimer < -30)
	cameraDown = 1
if (instance_exists(obj_e_precipitator))
	_id = collision_rectangle((x + 7), (y + 3), (x + 24), (y + 31), obj_desert, true, true)
else
	_id = collision_line((x + 7), (y + 3), (x + 24), (y + 31), obj_desert, true, true)
if (_id != -4)
{
	if place_free(x, y + 1)
		vspeed = 0.5
	pHSPEED = 1
	if obj_gameMgr.i_j
	{
		if (obj_gameMgr.playinput == 1)
		{
			vspeed = -4.7
			y -= 10
			gravity = 0.3
			audio_play_sound(se_jump, 10, false)
			instance_create(obj_player.x, obj_player.y, obj_jumpEffect)
		}
	}
}
else
	pHSPEED = 3
obj_bankiDownLine.x = x
obj_bankiDownLine.y = y
sukimaTimer--
if (icecreamTimer == 360)
{
	if (global.a_36 == 0)
	{
		ini_open(global.savedata)
		global.a_36 = 1
		ini_write_real("a", "36", 1)
		with (instance_create(x, y, obj_achievementGetMgr))
		{
			Timer = instance_number(obj_achievementGetMgr) * -300
			ac = 36
		}
		ini_close()
	}
}
up_check = 0
if (gamepad_axis_value(global.gamePad, gp_axislv) < (-global.stick) && reset == 1)
{
	up_check = 1
	reset = 0
}
if (gamepad_axis_value(global.gamePad, gp_axislv) == 0)
{
	up_check = 0
	reset = 1
}

var floorMove = collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, bbox_bottom + 3, obj_e_floorMove, 1, 1);
if (floorMove != noone && (vspeed >= 0 || vspeed >= floorMove.vspeed) && floorMove.bbox_top > bbox_bottom - 3) {
	vspeed = 0;
	if (floorMove.vspeed > 0) {
		y = min(y, floorMove.y - (bbox_bottom - y)) - 1;
		move_contact_solid(270, 4);
		y = floor(y);
		gravity = 0;
	} else if (floorMove.vspeed < 0 && !place_free(x, y - 1)) {
		e_kill_player(true);
	}
	if (collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, ceil(bbox_bottom) + 1, obj_e_floorMove, 1, 1)) {
		hspeed += floorMove.hspeed;
	}
}

obj_gameMgr.pushBlockL = 0
obj_gameMgr.pushBlockBlueL = 0
obj_gameMgr.pushBlockGreenL = 0
obj_gameMgr.pushBlockYellowL = 0
obj_gameMgr.pushBlockR = 0
obj_gameMgr.pushBlockBlueR = 0
obj_gameMgr.pushBlockGreenR = 0
obj_gameMgr.pushBlockYellowR = 0
