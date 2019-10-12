import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.sendRequest(findTestObject('User REST API Service/List Users'))

users_list = WS.sendRequestAndVerify(findTestObject('User REST API Service/List Users'))

WS.verifyElementPropertyValue(users_list, 'data[1].first_name', 'Lindsay')

WS.verifyElementPropertyValue(users_list, 'data[1].last_name', 'Ferguson')

WS.verifyElementPropertyValue(users_list, 'data[1].id', '8')

WS.verifyElementsCount(users_list, 'data', 6)

WS.verifyResponseStatusCodeInRange(users_list, 100, 200)

WS.verifyResponseStatusCode(users_list, 200)

