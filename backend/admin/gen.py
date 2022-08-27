a = """INSERT INTO users(
    id, username, email, authenticator, participate_in_leaderboards, individual_points, last_gained_points_at, is_active, is_admin, created_at)
VALUES (uuid_generate_v1(), 'fakerXX', 'fakerXX@gmail.com', 'google', true, XX, '2021-09-12 20:40:00-00', true, false, '2021-09-12 20:40:00-00');"""

for ii in range(0,100):
    print(a.replace("XX", str(ii)))