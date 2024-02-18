-- Your SQL goes here
CREATE TABLE `runs`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`distance` INTEGER NOT NULL,
	`duration` INTEGER NOT NULL
);

CREATE TABLE `records`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`user_id` INTEGER NOT NULL,
	`run_id` INTEGER NOT NULL,
	`distance` INTEGER NOT NULL,
	`duration` INTEGER NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	FOREIGN KEY (`run_id`) REFERENCES `runs`(`id`)
);

CREATE TABLE `users`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`username` TEXT NOT NULL,
	`email` TEXT NOT NULL
);

