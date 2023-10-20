--creation of main database

CREATE DATABASE main_site; 

select main_site; 


CREATE table meta_change_table ( 
 -- This table is for changes that have occured that needs to be processed. 
 -- This is done so that we don't have to process the whole database
    meta_change_Id INT NOT NULL AUTO_INCREMENT,
    meta_change_Modification_Date date,
    meta_change_Modification_Time time,
    table_name VARCHAR(255),
    table_Id INT
);

CREATE TABLE article_table (
    -- This is the main table for articles of the blog/site/whatever

    -- keys
    article_Id INT NOT NULL AUTO_INCREMENT,
    article_Author_Id INT, 
    site_config_Id INT,
    organization_Id INT,
    site_config_Id INT,

    article_Id_Hash VARCHAR(16),
    
    -- Markdown text, and meta data for documents
    article_Markdown_Text text,
    article_Title VARCHAR(255),
    article_Keywords VARCHAR(255),
    article_descriptions VARCHAR(255),
    article_uri VARCHAR(65536),
    -- date and time
    article_Creation_Date date,
    article_Creation_Time time,
    article_Modification_Date date,
    article_Modification_Time time,
    article_Publication_Date date,
    article_Publication_Time time,
    
    -- images
    article_thumbnail_Image MEDIUMBLOB,
    article_main_image_1 MEDIUMBLOB,
    article_main_image_2 MEDIUMBLOB,
    article_main_image_3 MEDIUMBLOB,
    article_main_image_4 MEDIUMBLOB,
    article_main_image_5 MEDIUMBLOB,
    article_main_image_6 MEDIUMBLOB,
    article_main_image_7 MEDIUMBLOB,
    article_main_image_8 MEDIUMBLOB,
    -- image descriptions
    article_main_image_description_1 VARCHAR(255),
    article_main_image_description_2 VARCHAR(255),
    article_main_image_description_3 VARCHAR(255),
    article_main_image_description_4 VARCHAR(255),
    article_main_image_description_5 VARCHAR(255),
    article_main_image_description_6 VARCHAR(255),
    article_main_image_description_7 VARCHAR(255),
    article_main_image_description_8 VARCHAR(255),
    -- html file name string. Spaces should be replaced by dashes
    article_html_file_name_string VARCHAR(255)

); 


CREATE TABLE author_table (
    author_Id INT NOT NULL AUTO_INCREMENT, 
    author_Id_Hash VARCHAR(16),
    author_First_Name VARCHAR(255),
    author_Middle_Name VARCHAR(255),
    author_Last_Name VARCHAR(255),
    author_uri VARCHAR(65536),
    author_email VARCHAR(255),
    author_description VARCHAR(1024) --longer than 255
);


CREATE TABLE organization_table (
    -- Main table for the organization that you may run!
    organization_Id INT NOT NULL AUTO_INCREMENT, 
    organization_Id_Hash VARCHAR(16),
    organization_Name VARCHAR(255),
    organization_Type VARCHAR(255),
    organization_Logo VARCHAR(255),
    organization_email VARCHAR(255),
    organization_description VARCHAR(255),
    organization_Uri VARCHAR(65536),
    organization_person_first_name VARCHAR(255),
    organization_person_middle_name VARCHAR(255),
    organization_person_last_name VARCHAR(255),
    organization_year_established VARCHAR(255)

);

CREATE TABLE site_config_table (
    -- global settings for all pages on the site
    site_config_Id INT NOT NULL AUTO_INCREMENT, 
    site_config_Id_Hash VARCHAR(16),
    site_config_header_color_top VARCHAR(48),
    site_config_header_color_bottom VARCHAR(48),
    site_config_footer_color_top VARCHAR(48),
    site_config_footer_color_bottom VARCHAR(48),
    site_config_background_color_top VARCHAR(48),
    site_config_background_color_bottom VARCHAR(48),
    site_config_main_org_name_color VARCHAR(48),
    site_config_content_color_top_and_bottom VARCHAR(48),
    site_config_content_color_middle VARCHAR(48),
    site_config_google_analytics_id VARCHAR(255),
    site_config_intro_sound_playback_delay_by_days TINYINT UNSIGNED,
    site_config_background_image MEDIUMBLOB,
    site_config_intro_sound MEDIUMBLOB,
    -- this is a base64 encoded image
    site_config_html_icon_base64 text
);


-- end
