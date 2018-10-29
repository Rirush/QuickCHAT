
.. _auth:

======================
/user: User Management
======================

.. _username-rules:

/checkUsername: Checking username availablility
===============================================

  **GET** ``/user/checkUsername``

    This method does not require authorization. This method does not return errors.

    Use this method to check whether username is valid and available for registation or not.

    .. note:: Valid username is a username that contains only latin charaters, numbers, underscores, and hyphens, doesn't start with underscore, and is between 3 and 32 characters long.
    
    Query arguments:

      +-----------+------------+-----------+---------------------------------+
      | Name      | Type       | Required? | Description                     |
      +===========+============+===========+=================================+
      | username  | String     | Yes       | Username that you want to check |
      +-----------+------------+-----------+---------------------------------+

    Returns:

      +-----------+-------------+-----------------+-------------------------------------------------------------+
      | Name      | Type        | Always present? | Description                                                 |
      +===========+=============+=================+=============================================================+
      | available | Boolean     | Yes             | Shows whether username is available for registration or not |
      +-----------+-------------+-----------------+-------------------------------------------------------------+

/register: Creating an account
==============================

  **POST** ``/user/register``

    This method does not require authorization. This method can return errors.

    Use this method to create new user account and acquire session token.

    Arguments:
    
      +------------+----------+-----------+----------------------------------------------------------------------+
      | Name       | Type     | Required? | Description                                                          |
      +============+==========+===========+======================================================================+
      | username   | String   | Yes       | Username that you want to take                                       |
      +------------+----------+-----------+----------------------------------------------------------------------+
      | password   | String   | Yes       | Password that you want to use. It should be longer than 8 characters |
      +------------+----------+-----------+----------------------------------------------------------------------+

    Returns:

      +---------------+-----------+-----------------+-----------------------------------------------------------------+
      | Name          | Type      | Always present? | Description                                                     |
      +===============+===========+=================+=================================================================+
      | user_id       | UUID      | Yes             | Unique 128-bit idenitifer of created user                       |
      +---------------+-----------+-----------------+-----------------------------------------------------------------+
      | session_token | String    | Yes             | Session token of created user.                                  |
      |               |           |                 | You will need it to do actions that require authorization.      |
      +---------------+-----------+-----------------+-----------------------------------------------------------------+

    Errors:

      +---------------------------+------------------------------------------------------------------------------------------+
      | Error code                | Cause                                                                                    |
      +===========================+==========================================================================================+
      | ``INVALID_USERNAME``      | Username does not meet username requirements described above. See :ref:`username-rules`. |
      +---------------------------+------------------------------------------------------------------------------------------+
      | ``INVALID_PASSWORD``      | Password does not meet password requirements described above.                            |
      +---------------------------+------------------------------------------------------------------------------------------+
      | ``USERNAME_TAKEN``        | Username is already taken by another user.                                               |
      +---------------------------+------------------------------------------------------------------------------------------+
      | ``INTERNAL_SERVER_ERROR`` | An internal server error occured while processing this request.                          |
      |                           | You should try your request later and notify administration.                             |
      +---------------------------+------------------------------------------------------------------------------------------+




