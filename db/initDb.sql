--main table for creating articles 

CREATE DATABASE main_site; 

select main_site; 


CREATE TABLE article_table (
    article_Id INT NOT NULL AUTO_INCREMENT,
    article_Author_Id INT, 
    site_config_Id INT,
    organization_Id INT,
    article_Id_Hash VARCHAR(16),
    article_Author_Hash VARCHAR(16),
    article_Markdown_Text text,
    article_Creation_Date date,
    article_Creation_Time time,
    article_Modification_Date date,
    article_Modification_Time time,
    article_Publication_Date date,
    article_Publication_Time time,
    article_Title VARCHAR(255),
    article_Meta_Data_Keywords VARCHAR(255),
    article_thumbnail_Image VARCHAR(255),
    article_main_image_1 VARCHAR(255),
    article_main_image_2 VARCHAR(255),
    article_main_image_3 VARCHAR(255),
    article_main_image_4 VARCHAR(255),
    article_main_image_5 VARCHAR(255),
    article_main_image_6 VARCHAR(255),
    article_main_image_7 VARCHAR(255),
    article_main_image_8 VARCHAR(255)

); 


CREATE TABLE author_table (
    author_Id INT NOT NULL AUTO_INCREMENT, 
    author_Id_Hash VARCHAR(16),
    author_First_Name VARCHAR(255),
    author_Middle_Name VARCHAR(255),
    author_Last_Name VARCHAR(255)
);


CREATE TABLE organization_table (
    organization_Id INT NOT NULL AUTO_INCREMENT, 
    organization_Id_Hash VARCHAR(16),
    organization_Name VARCHAR(255),
    organization_Type VARCHAR(255),
    organization_Logo VARCHAR(255),
    organization_Uri VARCHAR(255)
);

CREATE TABLE site_config_table (
    site_config_Id INT NOT NULL AUTO_INCREMENT, 
    site_config_Id_Hash VARCHAR(16),
    site_config_Id_Hash VARCHAR(16),
    site_config_image_max INT
);
