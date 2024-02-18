-- This file should undo anything in `up.sql`
ALTER TABLE `records` ADD COLUMN `distance` INTEGER NOT NULL;
ALTER TABLE `records` ADD COLUMN `duration` INTEGER NOT NULL;



