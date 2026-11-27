/*
Let's imagine we're building a photo-sharing app.
We'll add requirements one at a time and adjust
our design as each one arrives.

---

Define a `Photo` struct with three fields:
- `title` set to a String
- `likes` set to a usize
- `tags` set to a vector of Strings

Derive a Debug trait implementation.

Define an `impl` block for `Photo` with three
items:

1. A `new` constructor function that accepts a
title and returns a `Photo` with 0 likes and no
tags.

2. A `like` method that increases the likes by 1.

3. An `add_tag` method that accepts a tag and
adds it to the photo's tags.

---

Imagine product added this requirement: photos
must be organized into albums.

Define an `Album` struct with two fields:
- `name` set to a String
- `photos` set to a vector of `Photo` structs

Derive a Debug trait implementation.

Define an `impl` block for `Album` with three
items:

1. A `new` constructor function that accepts a
name and returns an `Album` with no photos.

2. An `add_photo` method that accepts a `Photo`
and adds it to the album.

3. A `total_likes` method that returns the sum
of the likes across all of the album's photos.

---

In `main`, create a "Sunset" photo and a
"Mountain" photo. Create a "Vacation" album and
add both photos.

---

Imagine product added this requirement: the same
photo can live in multiple albums. Create a
"Favorites" album and add the "Sunset" photo
to it.

Refactor your code to support this requirement.

Print the number of owners of each photo.
"Sunset" should have 3.
"Mountain" should have 2.

---

Imagine product added this requirement: users
can like a photo after it's been added to
albums. Every album that holds the photo should
reflect the change.

Refactor your code to support this requirement.

Then:

1. Like "Sunset" twice and "Mountain" once.

2. Print each album's total likes. "Vacation"
should show 3. "Favorites" should show 2.

---

Imagine product added this requirement: users
can tag a photo after it's been added to albums.
Every album that holds the photo should reflect
the change.

Refactor your code to support this requirement.

Then:

1. Tag "Sunset" with "beach" and "Mountain" with
"hiking".

2. Print both photos in Debug format to confirm
the likes and tags.

3. Print the "Favorites" album in Debug format.
*/
