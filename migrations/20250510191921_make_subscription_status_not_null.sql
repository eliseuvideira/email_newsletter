START TRANSACTION;

UPDATE subscriptions
   SET status = 'confirmed'
 WHERE status = null;

ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;

COMMIT;
