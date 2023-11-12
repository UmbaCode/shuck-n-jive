// json ldconstants

pub const JSONLD_WEBSITE: &str = r#"
<script type="application/ld+json">
{
    "@context": "https://schema.org",
    "@type": "WebSite",
    "url": "http://example.com/",
    "potentialAction": {
      "@type": "SearchAction",
      "target": "http://example.com/search?&q={query}",
      "query": "required"
    }
}
</script>
"#;

pub const JSONLD_ORGANIZATION: &str = r#"
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Organization",
  "url": "https://www.example.com",
  "logo": "https://www.example.com/images/logo.png"
}
</script>
"#;

pub const JSONLD_CORPORATION: &str = r#"
<script type='application/ld+json'>
{
  "@context": "http://www.schema.org",
  "@type": "Corporation",
  "name": "DataHotep Inc.",
  "logo": "null",
  "description": "Nu company",
  "address": {
     "@type": "PostalAddress",
     "addressLocality": "Tulsa",
     "addressRegion": "OK",
     "postalCode": "74153",
     "addressCountry": "United States"
  }
}
</script>
"#;

pub const JSONLD_VIDEO: &str = r#"
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "VideoObject",
  "name": "Introducing the self-driving bicycle in the Netherlands",
  "description": "This spring, Google is introducing the self-driving bicycle in Amsterdam, the world's premier cycling city. The Dutch cycle more than any other nation in the world, almost 900 kilometres per year per person, amounting to over 15 billion kilometres annually. The self-driving bicycle enables safe navigation through the city for Amsterdam residents, and furthers Google's ambition to improve urban mobility with technology. Google Netherlands takes enormous pride in the fact that a Dutch team worked on this innovation that will have great impact in their home country.",
  "thumbnailUrl": [
    "https://example.com/photos/1x1/photo.jpg",
    "https://example.com/photos/4x3/photo.jpg",
    "https://example.com/photos/16x9/photo.jpg"
   ],
  "uploadDate": "2016-03-31T08:00:00+08:00",
  "duration": "PT1M54S",
  "contentUrl": "https://www.example.com/video/123/file.mp4",
  "embedUrl": "https://www.example.com/embed/123",
  "interactionStatistic": {
    "@type": "InteractionCounter",
    "interactionType": { "@type": "WatchAction" },
    "userInteractionCount": 5647018
  },
  "regionsAllowed": "US,NL"
}
</script>
"#;

pub const JSONLD_PERSON: &str = r#"
<script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "Person",
      "address": {
        "@type": "PostalAddress",
        "addressLocality": "Colorado Springs",
        "addressRegion": "CO",
        "postalCode": "80840",
        "streetAddress": "100 Main Street"
      },
      "colleague": [
        "http://www.example.com/JohnColleague.html",
        "http://www.example.com/JameColleague.html"
      ],
      "email": "info@example.com",
      "image": "janedoe.jpg",
      "jobTitle": "Research Assistant",
      "name": "Jane Doe",
      "alumniOf": "Dartmouth",
      "birthPlace": "Philadelphia, PA",
      "birthDate": "1979-10-12",
      "height": "72 inches",
      "gender": "female",
      "memberOf": "Republican Party",
      "nationality": "Albanian",
      "telephone": "(123) 456-6789",
      "url": "http://www.example.com",
	    "sameAs" : [ "https://www.facebook.com/",
      "https://www.linkedin.com/",
      "http://twitter.com/",
      "http://instagram.com/",
      "https://plus.google.com/"]
    }
    </script>
"#;

pub const JSONLD_BREADCRUMB: &str = r#"
<script type="application/ld+json">
{
 "@context": "https://schema.org",
 "@type": "BreadcrumbList",
 "itemListElement":
 [
  {
   "@type": "ListItem",
   "position": 1,
   "item":
   {
    "@id": "https://example.com/dresses",
    "name": "Dresses"
    }
  },
  {
   "@type": "ListItem",
  "position": 2,
  "item":
   {
     "@id": "https://example.com/dresses/real",
     "name": "Real Dresses"
   }
  }
 ]
}
</script>
"#;

pub const JSONLD_ARTICLE: &str = r#"
<script type="application/ld+json">
{ "@context": "https://schema.org",
 "@type": "Article",
 "headline": "Extra! Extra! Read alla bout it",
 "alternativeHeadline": "This article is also about robots and stuff",
 "image": "http://example.com/image.jpg",
 "author": "Patrick Coombe",
 "award": "Best article ever written",
 "editor": "Craig Mount",
 "genre": "search engine optimization",
 "keywords": "seo sales b2b",
 "wordcount": "1120",
"publisher": {
    "@type": "Organization",
    "name": "Google",
    "logo": {
      "@type": "ImageObject",
      "url": "https://google.com/logo.jpg"
    }
  },
 "url": "http://www.example.com",
   "mainEntityOfPage": {
    "@type": "WebPage",
    "@id": "https://google.com/article"
  },
 "datePublished": "2015-09-20",
 "dateCreated": "2015-09-20",
 "dateModified": "2015-09-20",
 "description": "We love to do stuff to help people and stuff",
 "articleBody": "You can paste your entire post in here, and yes it can get really really long."
 }
</script>
"#;

// The graph method is what we want to utilize
pub const JSONLD_GRAPH_EXAMPLE: &str = r#"
<script type="application/ld+json">

{
 "@context": "http://schema.org",
  "@graph": [
{
 "@type": "Article",
 "headline": "Extra! Extra! Read alla bout it",
 "alternativeHeadline": "This article is also about robots and stuff",
 "image": "http://example.com/image.jpg",
 "author": "Patrick Coombe",
 "award": "Best article ever written",
 "editor": "Craig Mount",
 "genre": "search engine optimization",
 "keywords": "seo sales b2b",
 "wordcount": "1120",
"publisher": {
    "@type": "Organization",
    "name": "Google",
    "logo": {
      "@type": "ImageObject",
      "url": "https://google.com/logo.jpg"
    }
  },
 "url": "http://www.example.com",
   "mainEntityOfPage": {
    "@type": "WebPage",
    "@id": "https://google.com/article"
  },
 "datePublished": "2015-09-20",
 "dateCreated": "2015-09-20",
 "dateModified": "2015-09-20",
 "description": "We love to do stuff to help people and stuff",
 "articleBody": "You can paste your entire post in here, and yes it can get really really long."
 },

{

  "@type": "VideoObject",
  "name": "Introducing the self-driving bicycle in the Netherlands",
  "description": "This spring, Google is introducing the self-driving bicycle in Amsterdam, the world's premier cycling city. The Dutch cycle more than any other nation in the world, almost 900 kilometres per year per person, amounting to over 15 billion kilometres annually. The self-driving bicycle enables safe navigation through the city for Amsterdam residents, and furthers Google's ambition to improve urban mobility with technology. Google Netherlands takes enormous pride in the fact that a Dutch team worked on this innovation that will have great impact in their home country.",
  "thumbnailUrl": [
    "https://example.com/photos/1x1/photo.jpg",
    "https://example.com/photos/4x3/photo.jpg",
    "https://example.com/photos/16x9/photo.jpg"
   ],
  "uploadDate": "2016-03-31T08:00:00+08:00",
  "duration": "PT1M54S",
  "contentUrl": "https://www.example.com/video/123/file.mp4",
  "embedUrl": "https://www.example.com/embed/123",
  "interactionStatistic": {
    "@type": "InteractionCounter",
    "interactionType": { "@type": "WatchAction" },
    "userInteractionCount": 5647018
  },
  "regionsAllowed": "US,NL"
}
  ]
}
</script>
"#;
