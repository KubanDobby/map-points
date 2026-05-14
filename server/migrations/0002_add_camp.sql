ALTER TABLE points
ADD COLUMN camp TEXT NOT NULL
DEFAULT 'friend'
CHECK(camp IN ('friend', 'enemy'));