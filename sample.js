var authType = "nothing";


function idgisSetRedirectionCookie( theForm ) {


    if(theForm.usr_name != undefined){
        var upperStr = theForm.usr_name.value.toUpperCase();
        var normalStr = theForm.usr_name.value;

        if(upperStr != normalStr){
            window.location.href = "/"+"?Message=fail&GAURI="+theForm.HiddenURI.value;
            return false;
        }
    }
    var result = false;

    if ( isSingleClick() ) {
        index = theForm.AUTHMETHOD.selectedIndex;
        if(theForm.AUTHTYPE.value=="Cert"){
            return true;
        }

        if(index != undefined){
            if (theForm.AUTHMETHOD.options[index].value == "CERTIFICATE") {
                theForm.AUTHTYPE.value="Cert";
                return true;
            }
        }
        result = true;
    }

    return result;
}


<!-- Hide the script from the old browsers.

function findFocus()
{
    for(i=0;i<document.forms[0].elements.length;i++)
    {
        if(document.forms[0].elements[i].value != null &&
           document.forms[0].elements[i].value.length == 0)
        {
            document.forms[0].elements[i].focus();
            break;
        }
    }
}

var checkflag = 1;

function isSingleClick() {
    if (checkflag == 1) {
        checkflag = checkflag + 1
        return true;
    } else {
        return false;
    }
}

// End hiding-->
