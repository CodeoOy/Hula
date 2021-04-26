-- ----------------------------
-- Run this script manually to a new, fresh, and empty database to get a suitable demo data!
-- ----------------------------

-- ----------------------------
-- Records of skillcategories
-- ----------------------------
INSERT INTO "public"."skillcategories" VALUES ('e9becc32-0238-4561-b341-106de1c26040', 'Kielet', NULL, 'psi', '2021-04-23 18:10:28.28135', 'psi', '2021-04-23 19:22:41.623385', 5);
INSERT INTO "public"."skillcategories" VALUES ('e9becc32-0238-4561-b341-106de1c26041', 'Tekit', NULL, 'x', '2021-04-23 18:13:36.617682', 'psi', '2021-04-23 19:27:55.889668', 3);

-- ----------------------------
-- Records of skillscopes
-- ----------------------------
INSERT INTO "public"."skillscopes" VALUES ('e9becc32-0238-4561-b341-106de1c26042', '4-Step', 'psi', '2021-04-23 18:59:10.606994', 'psi', '2021-04-23 19:00:21.164389', 1);
INSERT INTO "public"."skillscopes" VALUES ('e9becc32-0238-4561-b341-106de1c26043', 'Kielet', 'psi', '2021-04-23 18:59:26.080963', 'psi', '2021-04-23 19:23:03.903862', 2);

-- ----------------------------
-- Records of skillscopelevels
-- ----------------------------
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26048', 'Guru', 'e9becc32-0238-4561-b341-106de1c26042', 4, 100, 'psi', '2021-04-23 19:02:20.5047', 'psi', '2021-04-23 19:04:11.032945', 2);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26045', 'Aloittelija', 'e9becc32-0238-4561-b341-106de1c26042', 1, 10, 'psi', '2021-04-23 19:01:29.667715', 'psi', '2021-04-23 19:23:15.432826', 2);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26046', 'Keskimääräinen', 'e9becc32-0238-4561-b341-106de1c26042', 2, 30, 'psi', '2021-04-23 19:01:45.093825', 'psi', '2021-04-23 19:23:20.677832', 3);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26047', 'Hyvä', 'e9becc32-0238-4561-b341-106de1c26042', 3, 70, 'psi', '2021-04-23 19:02:04.600572', 'psi', '2021-04-23 19:23:26.658751', 3);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26049', 'A1', 'e9becc32-0238-4561-b341-106de1c26043', 1, 10, 'psi', '2021-04-23 19:02:41.523011', 'psi', '2021-04-23 19:25:16.956395', 6);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26050', 'A2', 'e9becc32-0238-4561-b341-106de1c26043', 2, 20, 'psi', '2021-04-23 19:03:07.954804', 'psi', '2021-04-23 19:25:18.596356', 4);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26051', 'B1', 'e9becc32-0238-4561-b341-106de1c26043', 3, 35, 'psi', '2021-04-23 19:24:04.306249', 'psi', '2021-04-23 19:25:45.425443', 2);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26052', 'B2', 'e9becc32-0238-4561-b341-106de1c26043', 4, 50, 'psi', '2021-04-23 19:24:22.697999', 'psi', '2021-04-23 19:25:49.084675', 2);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26053', 'C1', 'e9becc32-0238-4561-b341-106de1c26043', 5, 75, 'psi', '2021-04-23 19:24:40.405509', 'psi', '2021-04-23 19:25:53.607845', 2);
INSERT INTO "public"."skillscopelevels" VALUES ('e9becc32-0238-4561-b341-106de1c26054', 'C2', 'e9becc32-0238-4561-b341-106de1c26043', 6, 100, 'psi', '2021-04-23 19:25:07.754666', 'psi', '2021-04-23 19:25:07.754666', 0);

-- ----------------------------
-- Records of skills
-- ----------------------------
INSERT INTO "public"."skills" VALUES ('e9becc32-0238-4561-b341-106de1c26060', 'Java', 'e9becc32-0238-4561-b341-106de1c26041', 'e9becc32-0238-4561-b341-106de1c26042', 'psi', '2021-04-23 19:27:36.948557', 'psi', '2021-04-23 19:30:09.753827', 1);
INSERT INTO "public"."skills" VALUES ('e9becc32-0238-4561-b341-106de1c26061', 'C#', 'e9becc32-0238-4561-b341-106de1c26041', 'e9becc32-0238-4561-b341-106de1c26042', 'psi', '2021-04-23 19:30:42.223038', 'psi', '2021-04-23 19:30:52.326599', 1);
INSERT INTO "public"."skills" VALUES ('e9becc32-0238-4561-b341-106de1c26561', 'Englanti', 'e9becc32-0238-4561-b341-106de1c26040', 'e9becc32-0238-4561-b341-106de1c26043', 'psi', '2021-04-26 11:15:40.187533', 'psi', '2021-04-26 11:15:40.187533', 0);
INSERT INTO "public"."skills" VALUES ('e9becc32-0238-4561-b341-106de1c26562', 'Ruotsi', 'e9becc32-0238-4561-b341-106de1c26040', 'e9becc32-0238-4561-b341-106de1c26043', 'psi', '2021-04-26 11:18:24.091643', 'psi', '2021-04-26 11:18:24.091643', 0);

-- ----------------------------
-- Records of projects
-- ----------------------------
INSERT INTO "public"."projects" VALUES ('e9becc32-0238-4561-b341-106de1c26080', 't', 'CodeMen');
INSERT INTO "public"."projects" VALUES ('e9becc32-0238-4561-b341-106de1c26380', 't', 'Solita');

-- ----------------------------
-- Records of projectneeds
-- ----------------------------
INSERT INTO "public"."projectneeds" VALUES ('e9becc32-0238-4561-b341-106de1c26680', 'e9becc32-0238-4561-b341-106de1c26080', 1, '2021-04-26 13:00:48', NULL, 100, 'psi', '2021-04-26 13:00:57.01757', 'psi', '2021-04-26 13:00:57.01757', 0);
INSERT INTO "public"."projectneeds" VALUES ('e9becc32-0238-4561-b341-106de1c26681', 'e9becc32-0238-4561-b341-106de1c26380', 1, '2021-04-05 13:18:36', NULL, 80, 'psi', '2021-04-26 13:18:45.761807', 'psi', '2021-04-26 13:18:45.761807', 0);

-- ----------------------------
-- Records of projectneedskills
-- ----------------------------
INSERT INTO "public"."projectneedskills" VALUES ('e9becc32-0238-4561-b341-106de1c26780', 'e9becc32-0238-4561-b341-106de1c26680', 'e9becc32-0238-4561-b341-106de1c26060', 'e9becc32-0238-4561-b341-106de1c26046', 10, 100, 'psi', '2021-04-26 13:01:57.181763', 'psi', '2021-04-26 13:01:57.181763', 0);
INSERT INTO "public"."projectneedskills" VALUES ('e9becc32-0238-4561-b341-106de1c26781', 'e9becc32-0238-4561-b341-106de1c26680', 'e9becc32-0238-4561-b341-106de1c26562', 'e9becc32-0238-4561-b341-106de1c26052', 0, 100, 'psi', '2021-04-26 13:14:48.662379', 'psi', '2021-04-26 13:14:48.662379', 0);
INSERT INTO "public"."projectneedskills" VALUES ('e9becc32-0238-4561-b341-106de1c26782', 'e9becc32-0238-4561-b341-106de1c26681', 'e9becc32-0238-4561-b341-106de1c26061', 'e9becc32-0238-4561-b341-106de1c26047', 0, 100, 'psi', '2021-04-26 13:20:31.271053', 'psi', '2021-04-26 13:20:31.271053', 0);

-- ----------------------------
-- Records of users
-- ----------------------------
INSERT INTO "public"."users" VALUES ('e9becc32-0238-4561-b341-106de1c26750', 'f', 't', 't', 'hessu.hopo@ankkalinna.fi', 'Hessu', 'Hopo', 'xxx', '2021-04-26 11:27:56');
INSERT INTO "public"."users" VALUES ('e9becc32-0238-4561-b341-106de1c26751', 'f', 't', 't', 'aku.ankka@ankkalinna.fi', 'Aku', 'Ankka', 'xxx', '2021-04-26 11:28:26');
INSERT INTO "public"."users" VALUES ('e9becc32-0238-4561-b341-106de1c26752', 'f', 't', 't', 'mikki.hiiri@ankkalinna.fi', 'Mikki', 'Hiiri', 'xxx', '2021-04-26 11:28:57');

-- ----------------------------
-- Records of userskills
-- ----------------------------
INSERT INTO "public"."userskills" VALUES ('e9becc32-0238-4561-b341-106de1c26850', 'e9becc32-0238-4561-b341-106de1c26750', 'e9becc32-0238-4561-b341-106de1c26060', 'e9becc32-0238-4561-b341-106de1c26048', 10, 'psi', '2021-04-24 22:30:47.278291', 'psi', '2021-04-26 11:30:28.818148', 6);
INSERT INTO "public"."userskills" VALUES ('e9becc32-0238-4561-b341-106de1c26851', 'e9becc32-0238-4561-b341-106de1c26750', 'e9becc32-0238-4561-b341-106de1c26061', 'e9becc32-0238-4561-b341-106de1c26048', 11, 'psi', '2021-04-24 22:34:29.815832', 'psi', '2021-04-26 11:30:30.827188', 6);
INSERT INTO "public"."userskills" VALUES ('e9becc32-0238-4561-b341-106de1c26852', 'e9becc32-0238-4561-b341-106de1c26750', 'e9becc32-0238-4561-b341-106de1c26561', 'e9becc32-0238-4561-b341-106de1c26053', NULL, 'psi', '2021-04-26 11:32:04.276846', 'psi', '2021-04-26 11:32:04.276846', 0);
INSERT INTO "public"."userskills" VALUES ('e9becc32-0238-4561-b341-106de1c26853', 'e9becc32-0238-4561-b341-106de1c26751', 'e9becc32-0238-4561-b341-106de1c26061', 'e9becc32-0238-4561-b341-106de1c26047', 3, 'psi', '2021-04-26 11:33:06.080314', 'psi', '2021-04-26 11:33:06.080314', 0);
INSERT INTO "public"."userskills" VALUES ('e9becc32-0238-4561-b341-106de1c26854', 'e9becc32-0238-4561-b341-106de1c26752', 'e9becc32-0238-4561-b341-106de1c26060', 'e9becc32-0238-4561-b341-106de1c26048', 15, 'psi', '2021-04-26 11:36:58.719168', 'psi', '2021-04-26 11:36:58.719168', 0);
INSERT INTO "public"."userskills" VALUES ('e9becc32-0238-4561-b341-106de1c26855', 'e9becc32-0238-4561-b341-106de1c26752', 'e9becc32-0238-4561-b341-106de1c26562', 'e9becc32-0238-4561-b341-106de1c26051', NULL, 'psi', '2021-04-26 11:37:53.897507', 'psi', '2021-04-26 11:37:53.897507', 0);

