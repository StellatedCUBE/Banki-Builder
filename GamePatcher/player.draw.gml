var px, py, scX, scY, rot, col, a;
if (global.character == "sekibanki")
{
	if (headHold == 0)
	{
		if (dir == 0)
			draw_sprite(global.bankiSprite, subImage, x, (pdy + 1))
		else
		{
			px = (x + sprite_width)
			py = y
			scX = -1
			scY = 1
			rot = 0
			col = c_white
			a = 1
			draw_sprite_ext(global.bankiSprite, subImage, px, (pdy + 1), scX, scY, rot, col, a)
		}
	}
	else if (headHold == 1)
	{
		if (dir == 0)
		{
			draw_sprite(global.bankiSprite_h, subImage, x, (pdy + 1))
			if (obj_holdHead.spring == 0)
			{
				if (global.bankiSprite == spr_banki_doremy)
					draw_sprite_ext(spr_head4, 3, x, ((pdy + 1) - 18), 1, 1, 0, c_white, 1)
				else
					draw_sprite_ext(spr_head4, 0, x, ((pdy + 1) - 18), 1, 1, 0, c_white, 1)
			}
			else if (obj_holdHead.spring == 1)
				draw_sprite_ext(spr_spring3, 0, x, ((pdy + 1) - 18), 1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 2)
				draw_sprite_ext(spr_spring4, 0, x, ((pdy + 1) - 18), 1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 3)
				draw_sprite_ext(spr_sukimaBall1, 0, x, ((pdy + 1) - 25), 1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 4)
				draw_sprite_ext(spr_sukimaBall2, 0, x, ((pdy + 1) - 25), 1, 1, 0, c_white, 1)
		}
		else
		{
			px = (x + sprite_width)
			py = y
			scX = -1
			scY = 1
			rot = 0
			col = c_white
			a = 1
			draw_sprite_ext(global.bankiSprite_h, subImage, px, (pdy + 1), scX, scY, rot, col, a)
			if (obj_holdHead.spring == 0)
			{
				if (global.bankiSprite == spr_banki_doremy)
					draw_sprite_ext(spr_head4, 3, (x + 32), ((pdy + 1) - 18), -1, 1, 0, c_white, 1)
				else
					draw_sprite_ext(spr_head4, 0, (x + 32), ((pdy + 1) - 18), -1, 1, 0, c_white, 1)
			}
			else if (obj_holdHead.spring == 1)
				draw_sprite_ext(spr_spring3, 0, (x + 32), ((pdy + 1) - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 2)
				draw_sprite_ext(spr_spring4, 0, (x + 32), ((pdy + 1) - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 3)
				draw_sprite_ext(spr_sukimaBall1, 0, (x + 32), ((pdy + 1) - 25), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 4)
				draw_sprite_ext(spr_sukimaBall2, 0, (x + 32), ((pdy + 1) - 25), -1, 1, 0, c_white, 1)
		}
	}
	else if (dir == 0)
		draw_sprite(global.bankiSprite, 6, x, (pdy + 1))
	else
	{
		px = (x + sprite_width)
		py = y
		scX = -1
		scY = 1
		rot = 0
		col = c_white
		a = 1
		draw_sprite_ext(global.bankiSprite, 6, px, (pdy + 1), scX, scY, rot, col, a)
	}
}
if (global.character == "rumia")
{
	if instance_exists(obj_e_playerBonkTimer)
	{
		if (dir == 0)
			draw_sprite(spr_rumiaOther, 0, x, (pdy + 1))
		else
			draw_sprite(spr_rumiaOtherL, 0, x, (pdy + 1))
	}
	else if !headHold
	{
		if (dir == 0)
			draw_sprite(spr_rumiaidle, subImage, x, (pdy + 1))
		else
			draw_sprite(spr_rumiaidleL, subImage, x, (pdy + 1))
	}
	else
	{
		if (subImage == 5)
			subImage = 4

		if dir {
			draw_sprite_ext(spr_e_rumiaL, subImage, x + 32, pdy + 1, -1, 1, 0, c_white, 1)
			if (subImage == 4)
				draw_sprite_ext(spr_e_rumiaHold, 0, x + 27, pdy + 2, -1, 1, 0, c_white, 1)
			else if (subImage == 2)
				draw_sprite_ext(spr_e_rumiaHold, 0, x + 27, pdy + 1, -1, 1, 0, c_white, 1)
			else
				draw_sprite_ext(spr_e_rumiaHold, 0, x + 27, pdy + 3, -1, 1, 0, c_white, 1)

			if (obj_holdHead.spring == 1)
				draw_sprite_ext(spr_spring3, 0, (x + 32), pdy + (1 - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 2)
				draw_sprite_ext(spr_spring4, 0, (x + 32), pdy + (1 - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 3)
				draw_sprite_ext(spr_sukimaBall1, 0, (x + 32), pdy + (1 - 25), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 4)
				draw_sprite_ext(spr_sukimaBall2, 0, (x + 32), pdy + (1 - 25), -1, 1, 0, c_white, 1)
		}

		else {
			draw_sprite(spr_e_rumiaL, subImage, x, (pdy + 1))
			if (subImage == 4)
				draw_sprite(spr_e_rumiaHold, 0, x + 5, pdy + 2)
			else if (subImage == 2)
				draw_sprite(spr_e_rumiaHold, 0, x + 5, pdy + 1)
			else
				draw_sprite(spr_e_rumiaHold, 0, x + 5, pdy + 3)
			
			if (obj_holdHead.spring == 1)
				draw_sprite(spr_spring3, 0, x, pdy + (1 - 18))
			else if (obj_holdHead.spring == 2)
				draw_sprite(spr_spring4, 0, x, pdy + (1 - 18))
			else if (obj_holdHead.spring == 3)
				draw_sprite(spr_sukimaBall1, 0, x, pdy + (1 - 25))
			else if (obj_holdHead.spring == 4)
				draw_sprite(spr_sukimaBall2, 0, x, pdy + (1 - 25))
		}
	}
}
if (global.character == "cirno")
{
	if (dir == 0)
		draw_sprite(spr_cirno, subImage, x, (pdy + 1))
	else
	{
		px = (x + sprite_width)
		py = y
		scX = -1
		scY = 1
		rot = 0
		col = c_white
		a = 1
		draw_sprite_ext(spr_cirno, subImage, px, (pdy + 1), scX, scY, rot, col, a)
	}
	if headHold {
		if dir {
			if (subImage == 4)
				draw_sprite_ext(spr_e_cirnoHold, 0, x + 28, pdy + 13, -1, 1, 0, c_white, 1)
			else if (subImage == 2 || subImage == 5)
				draw_sprite_ext(spr_e_cirnoHold, 0, x + 28, pdy + 12, -1, 1, 0, c_white, 1)
			else
				draw_sprite_ext(spr_e_cirnoHold, 0, x + 28, pdy + 14, -1, 1, 0, c_white, 1)

			if (obj_holdHead.spring == 1)
				draw_sprite_ext(spr_spring3, 0, (x + 32), pdy + (1 - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 2)
				draw_sprite_ext(spr_spring4, 0, (x + 32), pdy + (1 - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 3)
				draw_sprite_ext(spr_sukimaBall1, 0, (x + 32), pdy + (1 - 25), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 4)
				draw_sprite_ext(spr_sukimaBall2, 0, (x + 32), pdy + (1 - 25), -1, 1, 0, c_white, 1)
		}
		else {
			if (subImage == 4)
				draw_sprite(spr_e_cirnoHold, 0, x + 4, pdy + 13)
			else if (subImage == 2 || subImage == 5)
				draw_sprite(spr_e_cirnoHold, 0, x + 4, pdy + 12)
			else
				draw_sprite(spr_e_cirnoHold, 0, x + 4, pdy + 14)

			if (obj_holdHead.spring == 1)
				draw_sprite(spr_spring3, 0, x, pdy + (1 - 18))
			else if (obj_holdHead.spring == 2)
				draw_sprite(spr_spring4, 0, x, pdy + (1 - 18))
			else if (obj_holdHead.spring == 3)
				draw_sprite(spr_sukimaBall1, 0, x, pdy + (1 - 25))
			else if (obj_holdHead.spring == 4)
				draw_sprite(spr_sukimaBall2, 0, x, pdy + (1 - 25))
		}
	}
}
if (global.character == "seija")
{
	if instance_exists(obj_e_playerBonkTimer) {
		if dir
			draw_sprite_ext(spr_e_seijaBonk, 0, x + 28, pdy + 3, -1, 1, 0, c_white, 1)
		else
			draw_sprite(spr_e_seijaBonk, 0, x + 4, (pdy + 3))
	}
	else if (!headHold && !instance_exists(obj_magicBombSlow))
	{
		if (dir == 0)
			draw_sprite(spr_seija, subImage, x, (pdy + 1))
		else
		{
			px = (x + sprite_width)
			py = y
			scX = -1
			scY = 1
			rot = 0
			col = c_white
			a = 1
			draw_sprite_ext(spr_seija, subImage, px, (pdy + 1), scX, scY, rot, col, a)
		}
	}
	else if (dir == 0) {
		draw_sprite(spr_seijaHold, subImage, x, (pdy + 1))
		if headHold {
			if (obj_holdHead.spring == 1)
				draw_sprite(spr_spring3, 0, x, pdy + (1 - 18))
			else if (obj_holdHead.spring == 2)
				draw_sprite(spr_spring4, 0, x, pdy + (1 - 18))
			else if (obj_holdHead.spring == 3)
				draw_sprite(spr_sukimaBall1, 0, x, pdy + (1 - 25))
			else if (obj_holdHead.spring == 4)
				draw_sprite(spr_sukimaBall2, 0, x, pdy + (1 - 25))
		}
	}
	else
	{
		px = (x + sprite_width)
		py = y
		scX = -1
		scY = 1
		rot = 0
		col = c_white
		a = 1
		draw_sprite_ext(spr_seijaHold, subImage, px, (pdy + 1), scX, scY, rot, col, a)
		if headHold {
			if (obj_holdHead.spring == 1)
				draw_sprite_ext(spr_spring3, 0, (x + 32), pdy + (1 - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 2)
				draw_sprite_ext(spr_spring4, 0, (x + 32), pdy + (1 - 18), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 3)
				draw_sprite_ext(spr_sukimaBall1, 0, (x + 32), pdy + (1 - 25), -1, 1, 0, c_white, 1)
			else if (obj_holdHead.spring == 4)
				draw_sprite_ext(spr_sukimaBall2, 0, (x + 32), pdy + (1 - 25), -1, 1, 0, c_white, 1)
		}
	}
}
