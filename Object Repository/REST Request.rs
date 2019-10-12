<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>REST API Request Example</description>
   <name>REST Request</name>
   <tag></tag>
   <elementGuidId>1db0fc5b-dfc2-4fe7-a501-5bf542702205</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, 'data[1].first_name', 'Lindsay')
//assertThat(response.getResponseText()).contains('AddResult')


WS.verifyElementPropertyValue(response, 'data[1].last_name', 'Ferguson')


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)



assertThat(response.getResponseText()).contains('Lindsay')


//assertThat(response.getResponseText()).contains('Katalon Test Project')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
