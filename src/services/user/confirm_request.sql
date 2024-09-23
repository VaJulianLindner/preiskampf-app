UPDATE 
    contacts 
SET 
    contacts.state = 'confirmed'
WHERE 
    contacts.id = $1